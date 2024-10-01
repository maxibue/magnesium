# Magnesium
![mg](https://live.staticflickr.com/4634/38024207225_4667c270e4_n.jpg)

Magnesium is a pluggable image processing and hosting microservice with many quality-of-life features.

It was built in Rust using [Actix-Web](https://actix.rs/) & [MongoDB](https://www.mongodb.com/).

#### Features:
- Bucket support.
- Image resizing.
- Pictures served as 'webp' and/or only in original format.
- Images saved locally.

## Setup
*You need to have Rust/Cargo installed before obviously.*
- Download or clone the magnesium repository.
- Fill out the config.toml file.

**Here is a small list of all the settings in config.toml & their functions:**
| Setting's Name   | Standard Value                     | Explanation                                                                                                                                |
|------------------|------------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------|
| parent_directory | "test_setup"                       | It is the name of the directory where all the images & buckets are stored. It's located at './' of the executable.                         |
| buckets          | ["test_bucket_1", "test_bucket_2"] | These are the names of the buckets where images can be uploaded, stored & served. They are directories stored in the parent directory      |
| serve_as_webp    | true                               | This makes every image that is served in a link without an explicit file ending a webp image.                                              |
| allow_resizing   | true                               | This allows resizing of images when being uploaded.                                                                                        |
| allow_admin      | true                               | This allows the admin API endpoints that allow to add & remove API keys.                                                                   |
| db_name          | "magnesium_db"                     | This is the name of the MongoDB database that will be created/written to upon launch.                                                      |
| collection_name  | "keys"                             | This is the name of the MongoDB collection where the API keys will be stored.                                                              |

*(Keep in mind that the "test_setup" directory and its content are only there to give you an example of how the working Magnesium instance could look.)*

- Fill out all environment variables that are listed in the '.env.example' file.
- You can either add them directly into the environment or just remove the .example ending from the file.
- Run the `cargo build` command in the terminal. *(Go to the directory first.)*
- Use the outputted executable or run by using the `cargo run` command.

## Routes
- `/upload/{bucket}` - The route to upload to a specific bucket. (`API_KEY` header needed.)
- `/{bucket}/{filename}` - The route to serve a file from a specific bucket. (No API key required.)

Admin (disableable):

- `/keys/add` - The route to add an API key. (`ADMIN_KEY` header needed.)
- `/keys/remove` - The route to add an API key. (`ADMIN_KEY` header needed.)

Testing:

*If you have kept the test settings of `parent_directory` and `buckets` and you have kept the test_setup directory and its content you can test out Magnesium by going to the following routes:*

- `/test_bucket_1/test1` - The test image is served as a webp file.
- `/test_bucket_1/test1.png` - The test image is served as a png file.
- `/upload/test_bucket_1` - Upload an image and play around with Magnesium for yourself. (`API_KEY` header needed.)

*(Alternatively use `test_bucket_2` for a different bucket.*

## Resizing
If resizing is allowed you can optionally add a `width` and `height` header to the HTTP request to resize the image.

## Performance/How I used it
I have used Magnesium two times in larger projects. To see if it fits your needs I will explain how I used it and how it performed.
#### Where I used it:
I used it once for a Discord game bot where it served at most roughly 1000 images per second and it received about 70 images per second.

And it was used for approximately one month by a listing website that I freelanced for. (It was used as a quick solution before setting up the CDN.)
It served about 10.000-20.000 images per second with a rate of 30-40 images being uploaded every second.

So it is relatively field-tested for small and medium & larger projects.

#### How I implemented it:
The Discord bot was written in JavaScript and it would create battle pictures of the users internally and then send an HTTP request to my Magnesium instance and it would serve the response links in embeds.

I can not say too much about the listing site but it was structured like this: User uploads image -> Stored & waiting for approval -> (If approved) -> Send to Magnesium -> Attach response link to user in the database -> Serve on site.

#### Testing:
My testing showed it could theoretically **serve an estimated 50.000+ images per second** and handle image **uploads of up to 1.000 images per second**.

This should be enough for most projects/websites out there.

## Contributing
Feel free to create issues or pull requests if you want.
