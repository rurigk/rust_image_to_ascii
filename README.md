
# rust_image_to_ascii
A experiment to learn rust, Convert images to ascii

### Setup
Just install deps with

    $ cargo build
### Usage
Put the images you want to convert to ascii in the "images" directory and run the program with
         
    $ cargo run
    
### Troubleshooting

1. If you have artifacts with the generated ascii try using a image source with power-of-two resolution
	For example:
	512x512
	512x1024

2. If the program crashes is probable your image is corrupt or not supported, try opening the image with a image editor (like ms paint, gimp) and re-export the image to get a valid image
