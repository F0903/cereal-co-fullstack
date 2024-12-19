# Cereal Co. Backend

The backend for the week 10-11 Specialisterne assignment.

## Building Docker image

To build the Docker image, make sure you set up the required environment variables in the `.docker.env` file

The required environment variables are as follows:

- DATABASE_URL
- JWT_SECRET

### Populating database

After setup is done, the database is still not populated, you can populate the database with the example data in the Cereal.csv

To do this, first run the backend at least once to migrate the database, then follow the instructions in **migration/README**

## Running entity generation

Running entity gen:

> Run the following commands from the root dir of the backend

```sh
sea-orm-cli generate entity --with-serde both -o src/entities/
```
