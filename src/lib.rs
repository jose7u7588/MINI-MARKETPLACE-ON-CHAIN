use anchor_lang::prelude::*;

// Playground actualiza este ID automaticamente al hacer Build
declare_id!("2BWRjCKT4Ah34cWEpxTU8wxTLN8UX72iwUuvKg8rp5uX");

#[program]
pub mod marketplace {
    use super::*;

    ///////////////////////// Instruccion: Crear Tienda /////////////////////////
    /*
    Crea una PDA (Program Derived Address) unica por wallet.
    Guarda el bump en el struct para que Anchor pueda validar
    la PDA correctamente en todas las instrucciones siguientes.

    Parametros:
        * nombre      -> nombre de la tienda      -> String (max 60 chars)
        * descripcion -> descripcion de la tienda  -> String (max 200 chars)
    */
    pub fn crear_tienda(
        context: Context<NuevaTienda>,
        nombre: String,
        descripcion: String,
    ) -> Result<()> {
        let owner_id = context.accounts.owner.key();
        let bump = context.bumps.tienda; // Guardamos el bump de la PDA

        msg!("Creando tienda para owner: {}", owner_id);

        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            bump,
            nombre,
            descripcion,
            productos: Vec::new(),
        });

        msg!("Tienda creada exitosamente!");
        Ok(())
    }

    ///////////////////////// Instruccion: Agregar Producto /////////////////////////
    /*
    Agrega un nuevo producto al vector de productos de la tienda.
    Solo el owner de la tienda puede agregar productos.

    Parametros:
        * nombre      -> nombre del producto       -> String (max 60 chars)
        * descripcion -> descripcion del producto   -> String (max 200 chars)
        * precio      -> precio en lamports         -> u64
        * cantidad    -> unidades disponibles       -> u32
    */
    pub fn agregar_producto(
        context: Context<ModificarTienda>,
        nombre: String,
        descripcion: String,
        precio: u64,
        cantidad: u32,
    ) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let producto = Producto {
            nombre,
            descripcion,
            precio,
            cantidad,
            disponible: true,
        };

        context.accounts.tienda.productos.push(producto);
        msg!("Producto agregado exitosamente!");
        Ok(())
    }

    ///////////////////////// Instruccion: Actualizar Producto /////////////////////////
    /*
    Actualiza el precio y la cantidad de un producto existente.
    Busca el producto por nombre. Error si no existe.
    Solo el owner puede actualizar productos.

    Parametros:
        * nombre         -> nombre del producto a actualizar -> String
        * nuevo_precio   -> nuevo precio en lamports         -> u64
        * nueva_cantidad -> nuevas unidades disponibles      -> u32
    */
    pub fn actualizar_producto(
        context: Context<ModificarTienda>,
        nombre: String,
        nuevo_precio: u64,
        nueva_cantidad: u32,
    ) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.tienda.productos;

        for i in 0..productos.len() {
            if productos[i].nombre == nombre {
                productos[i].precio = nuevo_precio;
                productos[i].cantidad = nueva_cantidad;
                msg!(
                    "Producto '{}' actualizado: precio={} lamports, cantidad={}",
                    nombre,
                    nuevo_precio,
                    nueva_cantidad
                );
                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    ///////////////////////// Instruccion: Eliminar Producto /////////////////////////
    /*
    Elimina un producto del vector por su nombre.
    Error si el vector esta vacio o si el producto no existe.
    Solo el owner puede eliminar productos.

    Parametros:
        * nombre -> nombre del producto a eliminar -> String
    */
    pub fn eliminar_producto(
        context: Context<ModificarTienda>,
        nombre: String,
    ) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        require!(
            !context.accounts.tienda.productos.is_empty(),
            Errores::TiendaSinProductos
        );

        let productos = &mut context.accounts.tienda.productos;

        for i in 0..productos.len() {
            if productos[i].nombre == nombre {
                productos.remove(i);
                msg!("Producto '{}' eliminado exitosamente!", nombre);
                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    ///////////////////////// Instruccion: Alternar Disponibilidad /////////////////////////
    /*
    Cambia el estado de disponibilidad de un producto.
    true -> false o false -> true.
    Solo el owner puede alternar la disponibilidad.

    Parametros:
        * nombre -> nombre del producto -> String
    */
    pub fn alternar_disponibilidad(
        context: Context<ModificarTienda>,
        nombre: String,
    ) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.tienda.productos;

        for i in 0..productos.len() {
            if productos[i].nombre == nombre {
                let nuevo_estado = !productos[i].disponible;
                productos[i].disponible = nuevo_estado;
                msg!(
                    "Producto '{}' ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );
                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    ///////////////////////// Instruccion: Ver Productos /////////////////////////
    /*
    Muestra en el log todos los productos de la tienda.
    Solo el owner puede ver sus productos.

    Parametros:
        Ninguno
    */
    pub fn ver_productos(context: Context<ModificarTienda>) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!(
            "Productos en la tienda '{}': {:#?}",
            context.accounts.tienda.nombre,
            context.accounts.tienda.productos
        );

        Ok(())
    }
}

///////////////////////// Codigos de Error /////////////////////////
#[error_code]
pub enum Errores {
    #[msg("Error: No eres el propietario de esta tienda")]
    NoEresElOwner,

    #[msg("Error: El producto que buscas no existe en la tienda")]
    ProductoNoExiste,

    #[msg("Error: La tienda no tiene productos registrados")]
    TiendaSinProductos,
}

///////////////////////// Structs /////////////////////////

// Cuenta principal almacenada on-chain (PDA unica por wallet)
#[account]
#[derive(InitSpace)]
pub struct Tienda {
    pub owner: Pubkey,  // Llave publica del dueno (32 bytes)
    pub bump: u8,       // Bump de la PDA, necesario para validarla

    #[max_len(60)]
    pub nombre: String, // Nombre de la tienda

    #[max_len(200)]
    pub descripcion: String, // Descripcion de la tienda

    #[max_len(10)] // Maximo 10 productos por tienda
    pub productos: Vec<Producto>,
}

// Struct interno (no es una cuenta), representa un producto del marketplace
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Producto {
    #[max_len(60)]
    pub nombre: String, // Nombre del producto

    #[max_len(200)]
    pub descripcion: String, // Descripcion del producto

    pub precio: u64,    // Precio en lamports (1 SOL = 1_000_000_000 lamports)
    pub cantidad: u32,  // Unidades disponibles en inventario
    pub disponible: bool, // true = activo, false = inactivo
}

///////////////////////// Contextos /////////////////////////

// Contexto para crear una nueva tienda (PDA)
#[derive(Accounts)]
pub struct NuevaTienda<'info> {
    #[account(mut)]
    pub owner: Signer<'info>, // Firma y paga la transaccion

    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8, // +8 por el discriminador de Anchor
        seeds = [b"tienda", owner.key().as_ref()], // PDA unica por wallet
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    pub system_program: Program<'info, System>, // Necesario para crear cuentas
}

// Contexto para todas las instrucciones que modifican o leen la tienda
#[derive(Accounts)]
pub struct ModificarTienda<'info> {
    pub owner: Signer<'info>, // El owner firma la transaccion

    #[account(
        mut,
        seeds = [b"tienda", owner.key().as_ref()], // Valida que sea la PDA correcta
        bump = tienda.bump                          // Usa el bump guardado en el struct
    )]
    pub tienda: Account<'info, Tienda>,
}
