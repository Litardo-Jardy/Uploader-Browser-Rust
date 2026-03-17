<div align="center" >
🦀 Uploader-Browser - API-REST
</div>

## Descripcion 

Este proyecto nace con el objetivo de facilitar el proceso de compartir archivos entre un dispositivo móvil y una computadora de una forma más rápida, eficiente y personalizada, sin depender de aplicaciones de terceros.
Actualmente existen varias formas de transferir archivos, como el uso de aplicaciones de mensajería (por ejemplo WhatsApp). Sin embargo, estas soluciones presentan varias limitaciones. En primer lugar, suelen tener restricciones en el 
tamaño o tipo de archivo que se puede enviar. En segundo lugar, los archivos terminan almacenados de forma poco organizada cuando llegan al computador, lo que obliga a dedicar tiempo adicional para ordenarlos. Finalmente, también existe la preocupación por la privacidad al depender de servicios externos.
La idea de este proyecto es desarrollar un backend que interactúe directamente con el sistema de archivos del servidor y permita gestionar archivos y carpetas de forma estructurada. Este backend expone una API REST que posteriormente será consumida por un frontend desarrollado en React.js, el cual proporcionará una interfaz amigable para subir, organizar y gestionar archivos de manera sencilla.


## Informacion tecnica

Puedes encontrar mas informacin del proyecto sobre la arquitectura, manejo de errores, el manejo de archivos, etc en mis posts de [Linkedin](https://www.linkedin.com/posts/jardy-litardo-vera-209a36219_estoy-construyendo-una-api-rest-para-la-gestion-activity-7438075293967413248-BUxn?utm_source=share&utm_medium=member_desktop&rcm=ACoAADcl5KkBgd0pPFb3GBmDmrocdAmOv4mv63k)

# Instalación y ejecución

### 1. Clonar el repositorio  
```bash

   https://github.com/Litardo-Jardy/Uploader-Browser-Rust.git

```

### 2. Restaurar dependencias
```bash

   cargo build

```

### 3. Crear .env apartir de .env.example para las variables de entorno
```bash

   BASE_DIR=la-ruta-sobre-la-que-se-va-a-trabajar
   SECRET=tu-firma-para-el-token
   USERR=tu-usuario-para-login
   PASS=tu-contraseña-para-login

```

### 4. Ejecutar del proyecto
```bash

   cargo run

```

# Autor

- Jardy Litardo [Litardo-Jardy](https://github.com/Litardo-Jardy)
