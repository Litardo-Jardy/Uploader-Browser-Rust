<div align="center" >
🦀 Uploader-Browser - API-REST
</div>

## Description:

Este proyecto nace con el objetivo de facilitar el proceso de compartir archivos entre un dispositivo móvil y una computadora de una forma más rápida, eficiente y personalizada, sin depender de aplicaciones de terceros.

Actualmente existen varias formas de transferir archivos, como el uso de aplicaciones de mensajería (por ejemplo WhatsApp). Sin embargo, estas soluciones presentan varias limitaciones. En primer lugar, suelen tener restricciones en el 
tamaño o tipo de archivo que se puede enviar. En segundo lugar, los archivos terminan almacenados de forma poco organizada cuando llegan al computador, lo que obliga a dedicar tiempo adicional para ordenarlos. Finalmente, también existe la preocupación por la privacidad al depender de servicios externos.
La idea de este proyecto es desarrollar un backend que interactúe directamente con el sistema de archivos del servidor y permita gestionar archivos y carpetas de forma estructurada. Este backend expone una API REST que posteriormente será consumida por un frontend desarrollado en React.js, el cual proporcionará una interfaz amigable para subir, organizar y gestionar archivos de manera sencilla.


## Tech info:

You can find more information about this project, such as the architecture, error management and file management used it's in my LinkedIn posts [Linkedin](https://www.linkedin.com/posts/jardy-litardo-vera-209a36219_estoy-construyendo-una-api-rest-para-la-gestion-activity-7438075293967413248-BUxn?utm_source=share&utm_medium=member_desktop&rcm=ACoAADcl5KkBgd0pPFb3GBmDmrocdAmOv4mv63k)

## Installation & Execution:

### 1. Clone repository
```bash

   https://github.com/Litardo-Jardy/Uploader-Browser-Rust.git

```

### 2. Build dependencies
```bash

   cargo build

```

### 3. Create .env file from .env.example to store environment variables
```bash

   BASE_DIR=la-ruta-sobre-la-que-se-va-a-trabajar
   SECRET=tu-firma-para-el-token
   USERR=tu-usuario-para-login
   PASS=tu-contraseña-para-login

```

### 4. Execute project
```bash

   cargo run

```

## Endpoints:

### Login:

```bash
      curl -s -X POST http://localhost:3000/login \
         -H "Content-Type: application/json" \
         -d '{"user": "", "pass": ""}'
```

### List folders (Authenticate):

```bash
      curl -v http://localhost:3000/list_folders?path="*" \
          -H "Authorization: Bearer "
```

### List files (Authenticate):


```bash
      curl -v http://localhost:3000/list_files?path="*" \
           -H "Authorization: Bearer "
```

### Delete folder (Authenticate):

```bash
      curl -X POST http://localhost:3000/delete_folder \
           -H "Content-Type: application/json" \
           -H "Authorization: Bearer " \
           -d '{"path": ""}'
```

### Delete file (Authenticate):

```bash
       curl -X POST http://localhost:3000/delete_file \
           -H "Content-Type: application/json" \
           -H "Authorization: Bearer " \
           -d '{"path": ""}'
```

### Edit File/Folder (Authenticate):

```bash
        curl -v -X POST http://localhost:3000/edit_element \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer " \
            -d '{"path": "", "new_path": ""}'
```

### Create folder (Authenticate):

```bash
        curl -X POST http://localhost:3000/create_folder \
            -H "Content-Type: application/json" \
            -H "Authorization: Bearer " \
            -d '{"path": ""}'
```

### Upload File (Authenticate):

```bash
        curl -X POST http://localhost:3000/upload_file \
             -H "Authorization: Bearer " \
             -F "name=" \
             -F "route=" \
             -F "file=@/"
```
# Author:

- Jardy Litardo [Litardo-Jardy](https://github.com/Litardo-Jardy)
