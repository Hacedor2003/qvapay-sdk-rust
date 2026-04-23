# QvaPay SDK for Rust

Este es un SDK no oficial para interactuar con la API de [QvaPay](https://qvapay.com). Permite gestionar pagos, consultar balances, realizar operaciones con acciones (stocks) y manejar la autenticación de usuarios.

## Características

- ✅ Cliente asíncrono basado en `reqwest`.
- ✅ Manejo de errores detallado con `thiserror`.
- ✅ Modelos de datos completos con `serde`.
- ✅ Soporte para entornos de Sandbox y Producción.
- ✅ Cobertura de endpoints de Pagos, Merchant, Stocks y Auth.

## Instalación

Añade esto a tu `Cargo.toml`:

```toml
[dependencies]
qvapay-sdk = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## Uso Rápido

### Crear una Factura (Pago)

```rust
use qvapay_sdk::{QvaPayClient, Environment, models::CreateInvoiceRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = QvaPayClient::new("tu_api_key".to_string(), Environment::Sandbox);

    let invoice_req = CreateInvoiceRequest {
        amount: 5.0,
        description: "Suscripción Premium".to_string(),
        remote_id: "order_999".to_string(),
        webhook: Some("https://tu-sitio.com/webhook".to_string()),
        products: None,
        expire_at: None,
    };

    let invoice = client.create_invoice(&invoice_req).await?;
    println!("Por favor, paga aquí: {}", invoice.url);

    Ok(())
}
```

### Consultar Balance

```rust
let balance = client.get_balance().await?;
println!("Tu balance actual es: {} USD", balance.balance);
```

### Operaciones con Stocks

```rust
// Listar acciones
let stocks = client.get_stocks().await?;

// Comprar acciones de Apple (AAPL)
let trade = client.buy_stock("AAPL", 50.0).await?;
println!("Compra exitosa: {}", trade.trade_uuid);
```

## Estructura del Proyecto

- `src/client.rs`: Cliente principal y configuración de red.
- `src/api/`: Implementación de los endpoints por categorías (auth, pagos, stocks, merchant).
- `src/models/`: Estructuras de datos para peticiones y respuestas.
- `src/error.rs`: Definición de errores personalizados.

## Pruebas

Para ejecutar las pruebas:

```bash
cargo test
```

## Cómo publicar en Crates.io

1. **Crea una cuenta** en [crates.io](https://crates.io) y obtén tu API token.
2. **Inicia sesión** en tu terminal:
   ```bash
   cargo login <tu_token>
   ```
3. **Verifica el paquete**: Asegúrate de que todo esté en orden y pase los tests.
   ```bash
   cargo package
   ```
4. **Publica**:
   ```bash
   cargo publish
   ```

---

Desarrollado por [Ciberguajiro](https://github.com/ciberguajiro).
