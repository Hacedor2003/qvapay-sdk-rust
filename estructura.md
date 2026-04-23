qvapay-sdk/
├── Cargo.toml
└── src/
    ├── lib.rs           # Punto de entrada principal que reexporta todo
    ├── client.rs        # Cliente principal, configuración y lógica de conexión
    ├── error.rs         # Definición de errores personalizados (usando thiserror)
    ├── auth.rs          # Lógica de autenticación (ej. manejo de API Keys)
    ├── api/             # Módulo para los endpoints específicos de la API
    │   ├── mod.rs
    │   ├── pagos.rs     # Funciones para crear, consultar, reembolsar pagos
    │   └── clientes.rs  # Funciones para gestionar clientes (si aplica)
    ├── models/          # Estructuras de datos (DTOs)
    │   ├── mod.rs
    │   ├── pago.rs      # Struct Pago, etc.
    │   └── cliente.rs   # Struct Cliente
    └── utils.rs         # Funciones de utilería (ej. formateo de fechas)