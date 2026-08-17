<div align="center" >
🦀 Uploader-Browser - API-REST
</div>

## Description:

This project was created to make it easier, faster, and more efficient to share files between a mobile device and a computer, without relying on third-party apps.

There are already several ways to transfer files, such as messaging apps like WhatsApp — but these come with real limitations. For one, they often restrict file size or type. On top of that, files tend to arrive on the computer in a disorganized way, forcing users to spend extra time sorting them out. And finally, there's the privacy concern that comes with depending on external services.

The goal of this project is to build a backend that interacts directly with the server's file system, allowing files and folders to be managed in a structured way. This backend exposes a REST API, which will later be consumed by a React.js frontend that provides a clean, user-friendly interface for uploading, organizing, and managing files.


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
