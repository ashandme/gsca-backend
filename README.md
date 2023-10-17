# GSCA BACKEND

backend para gsca escrito en rust

## Tabla de Contenidos

- [Requisitos](#requisitos)
- [Configuración](#configuración)
- [Ejecución](#ejecución)
- [Uso](#uso)
- [API](#api)
- [Base de Datos](#base-de-datos)
- [Contribuciones](#contribuciones)

## Requisitos

Se necesita Rust y [diesel_cli](https://diesel.rs/) instalado. También una base de datos :)

## Configuración

Solo requiere que ponga la url de la base de datos en `.env`

```
DATABASE_URL="mysql://admin:password@localhost/base"
```

luego realice las migraciones con `diesel migration run`
## Ejecución

`cargo run`

## Uso

Se usa con el [frontend](https://github.com/ashandme/gsca-frontend)

## API

La documentación es privada por el momento.

## Base de Datos

Solo soporta MySQL pero es fácil portearlo para otras bases de datos como PostgreSQL

## Contribuciones

Alexander: Base de datos
Camila: Frontend
Nico: Documentación y Base de datos