# Mini Marketplace On-Chain

Programa backend desarrollado en **Rust con Anchor Framework** sobre la blockchain de **Solana**.  
Permite a cualquier wallet crear su propia tienda on-chain y gestionar un catálogo de productos mediante un CRUD completo.

---

## ¿Qué hace este proyecto?

Un vendedor puede:

- Crear su tienda personal almacenada en una **PDA** única por wallet
- Agregar productos con nombre, descripción, precio y cantidad
- Actualizar el precio y stock de cualquier producto existente
- Eliminar productos del catálogo por nombre
- Activar o desactivar la disponibilidad de un producto
- Ver todos sus productos en el log de la transacción

> Este proyecto es únicamente el **backend** del marketplace (programa on-chain). No incluye transferencias de SOL ni interfaz de usuario.

---

## Instrucciones del Programa

| Instrucción | Descripción | Parámetros |
|---|---|---|
| `crear_tienda` | Inicializa la PDA de la tienda | `nombre`, `descripcion` |
| `agregar_producto` | Agrega un producto al vector | `nombre`, `descripcion`, `precio`, `cantidad` |
| `actualizar_producto` | Edita precio y cantidad por nombre | `nombre`, `nuevo_precio`, `nueva_cantidad` |
| `eliminar_producto` | Elimina un producto por nombre | `nombre` |
| `alternar_disponibilidad` | Toggle disponible true/false | `nombre` |
| `ver_productos` | Muestra todos los productos en el log | ninguno |

---

## Estructura de Datos

### Tienda (Cuenta PDA)

| Campo | Tipo | Descripción |
|---|---|---|
| `owner` | `Pubkey` | Llave pública del dueño (32 bytes) |
| `bump` | `u8` | Bump de la PDA para validación |
| `nombre` | `String` | Nombre de la tienda (max 60 chars) |
| `descripcion` | `String` | Descripción de la tienda (max 200 chars) |
| `productos` | `Vec<Producto>` | Vector de productos (max 10) |

### Producto (Struct interno)

| Campo | Tipo | Descripción |
|---|---|---|
| `nombre` | `String` | Nombre del producto (max 60 chars) |
| `descripcion` | `String` | Descripción del producto (max 200 chars) |
| `precio` | `u64` | Precio en lamports (1 SOL = 1,000,000,000 lamports) |
| `cantidad` | `u32` | Unidades disponibles en inventario |
| `disponible` | `bool` | `true` = activo, `false` = inactivo |

---

## Códigos de Error

| Error | Descripción |
|---|---|
| `NoEresElOwner` | El caller no es dueño de la tienda |
| `ProductoNoExiste` | El producto buscado no existe en el vector |
| `TiendaSinProductos` | La tienda no tiene productos registrados |

---

## Cómo usar en Solana Playground

### 1. Abrir el proyecto

1. Ve a [beta.solpg.io](https://beta.solpg.io)
2. Crea un nuevo proyecto Anchor llamado `mini-marketplace`
3. Reemplaza el contenido de `src/lib.rs` con el código de este repositorio
4. Click en **Build** — Playground asigna el Program ID automáticamente
5. Click en **Deploy**

### 2. Obtener SOL de devnet gratis

El programa se despliega en devnet (red de pruebas). Para obtener SOL gratuito:

- Dentro de Playground: click en **Get Airdrop** (botón inferior izquierdo)
- O visita [faucet.solana.com](https://faucet.solana.com) y pega tu wallet address

### 3. Calcular la dirección de tu PDA

Antes de llamar cualquier instrucción necesitas la dirección de tu PDA.  
En la pestaña **`client.ts`** de Playground pega esto y haz click en **Run**:

```typescript
const [tiendaPDA] = web3.PublicKey.findProgramAddressSync(
  [Buffer.from("tienda"), pg.wallet.publicKey.toBuffer()],
  pg.PROGRAM_ID
);
console.log("Tu PDA:", tiendaPDA.toString());
```

Copia la dirección que aparece — la usarás en todas las instrucciones.

### 4. Orden recomendado para probar

```
crear_tienda → agregar_producto → actualizar_producto / alternar_disponibilidad / ver_productos → eliminar_producto
```

> Siempre llama `crear_tienda` primero. Si llamas otra instrucción antes, fallará porque la PDA no existe todavía.

---

## Seguridad

Todas las instrucciones validan con `require!` que el caller sea el owner de la PDA:

```rust
require!(
    context.accounts.tienda.owner == context.accounts.owner.key(),
    Errores::NoEresElOwner
);
```

La PDA se valida en cada instrucción usando los seeds `["tienda", owner.key()]` y el `bump` guardado en el struct `Tienda`, garantizando que solo el wallet creador puede interactuar con su tienda.


## Tecnologías

| Tecnología | Versión | Uso |
|---|---|---|
| Rust | stable | Lenguaje de programación principal |
| Anchor Framework | 0.30.1 | Framework para desarrollo en Solana |
| Solana | Devnet | Blockchain donde se despliega el programa |
| Solana Playground | beta | Entorno de desarrollo en el navegador |

---

## Autor

Proyecto desarrollado como requisito de certificación del curso **WayLearning — Desarrollo en Solana**.
