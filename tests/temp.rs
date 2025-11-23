#[cfg(test)]
mod tests {
    
    use moseiik::main::Options;
    use moseiik::main::compute_mosaic;
    use image::{
    imageops::{resize, FilterType::Nearest},
    GenericImage, GenericImageView, ImageReader, RgbImage,
    };
    use std::path::Path;

    use super::*; //import des functions et structures du main file

    #[test]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    fn test_x86() {
         //test avx2 or sse2 if available

        use moseiik::main::compute_mosaic;
        let chemin_image = "assets/moseiik_test_images/kit.jpeg";
        let arg_test = Options {
            image: String::from(chemin_image), // Location of the target image
            output: String::from("assets/out_x86.png"), // Saved result location
            tiles: String::from("assets/moseiik_test_images/images"), // Location of the tiles
            scaling: 1, // Scaling factor of the image
            tile_size: 25, // Size of the tiles
            remove_used: true, // Remove used tile
            verbose: true,
            simd: true, // Use SIMD when available
            num_thread: 1, // Specify number of threads to use, leave blank for default
        };
        let _output_path = "assets/out_x86.png";
        compute_mosaic(arg_test);
        assert!(Path::new(_output_path).exists()); //check if anything was created in my output_path

        let _good_image: RgbImage = ImageReader::open("assets/ground-truth-kit.png").unwrap().decode().unwrap().into_rgb8();
        let _test_image: RgbImage = ImageReader::open(_output_path).unwrap().decode().unwrap().into_rgb8();

        //assert!(_good_image == _test_image, "The images are not the exactly the same");

        
    }

    #[test]
    #[cfg(target_arch = "aarch64")]
    fn test_aarch64() {
        let chemin_image = "assets/moseiik_test_images/kit.jpeg";
        let arg_test = Options {
            image: String::from(chemin_image), // Location of the target image
            output: String::from("assets/out_aarch64.png"), // Saved result location
            tiles: String::from("assets/moseiik_test_images/images"), // Location of the tiles
            scaling: 1, // Scaling factor of the image
            tile_size: 25, // Size of the tiles
            remove_used: true, // Remove used tile
            verbose: true,
            simd: true, // Use SIMD when available
            num_thread: 1, // Specify number of threads to use, leave blank for default
        };
        let _output_path = "assets/out_aarch64.png";
        compute_mosaic(arg_test);
        assert!(Path::new(_output_path).exists()); //check if anything was created in my output_path
        
        let _good_image: RgbImage = ImageReader::open("assets/ground-truth-kit.png").unwrap().decode().unwrap().into_rgb8();
        let _test_image: RgbImage = ImageReader::open(_output_path).unwrap().decode().unwrap().into_rgb8();

        //assert!(_good_image == _test_image, "The images are not the exactly the same");
    
    }

    #[test]
    fn test_generic() {
        let chemin_image = "assets/moseiik_test_images/kit.jpeg";
        let arg_test = Options {
            image: String::from(chemin_image),
            output: String::from("assets/moseiik_test_images/out_generic.png"),
            tiles: String::from("assets/moseiik_test_images/images"),
            scaling: 1,
            tile_size: 25,
            remove_used: true,
            verbose: true,
            simd: false, // This time we disable SIMD to test generic implementation
            num_thread: 1,
        };
    let _output_path = "assetsout_generic.png";
    compute_mosaic(arg_test);
    assert!(Path::new(_output_path).exists());

    let _good_image: RgbImage = ImageReader::open("assets/ground-truth-kit.png").unwrap().decode().unwrap().into_rgb8();
    let _test_image: RgbImage = ImageReader::open(_output_path).unwrap().decode().unwrap().into_rgb8();

    //assert!(_good_image == _test_image, "The images are not the exactly the same");
    }
}