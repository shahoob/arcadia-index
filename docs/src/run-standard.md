# Standard Setup

This is the 'regular' way to run Arcadia by installing dependencies directly on your system.

## Database Setup

Spawn an instance of postgresql and run the migrations (`/backend/migrations/*.sql`). For more information, refer to the database part of the [docker section](run-docker.md).

## Backend Setup

Create a `.env` file from the template and update the values if needed:

```bash
cp .env.example .env
```

This command will download the dependencies, build them and build arcadia, as well as run it:

```bash
cargo run
```

If you encounter some errors, it is probably because some OS dependencies are missing. Install them and run the command again.

## Frontend Setup

Create a `.env` file from the template and update the values if needed:

```bash
cp .env.example .env
```

Install the dependencies:

```bash
npm install
```

Run the frontend:

```bash
npm run dev
```
