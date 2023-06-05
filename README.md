# `🚀 GEN T API`
This api was developed only for educational purposes. Not recommended to use because it's potencially unstable.

## `📡 Endpoints`
Method | Endpoint | Description | Need Token | Query params |
| :------: | :------: | ------- | :------: | :------: |
| `GET` | `/api/v1/gifs`  | Returns all the gifs with a max limit of `50` gifs (default `25`) | ✅ | `limit` & `page` |
| `GET` | `/api/v1/gifs/:id`  | Returns an specific gif. | ✅ | - |
| `POST` | `/api/v1/gifs` | Post a gif specified in the body. | ✅ | - |
| `POST` | `/api/v1/auth` | Generates a new Authorization token to interact with the API. | ❌ | - |
 
