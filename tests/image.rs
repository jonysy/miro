#![feature(plugin)]
#![plugin(speculate)]

extern crate image;
extern crate miro;

speculate! {

    describe "an integral image" {
        use image::GrayImage;
        use miro::image::IntegralImage;

        
    }

    describe "a pyramid" {
        use image::GrayImage;
        use miro::image::Pyramid;

        it "contains the correct number of layers" {
            let gray_image = GrayImage::new(1, 1);
            let pyramid = Pyramid::new(gray_image, 2, |prev| prev.clone());

            assert_eq!(pyramid.len(), 2);
        }

        it "has the correct dimensions" {
            let gray_image = GrayImage::new(2, 2);
            let pyramid = Pyramid::new(gray_image, 2, |_| GrayImage::new(1, 1));
            let (w, h) = pyramid[1].dimensions();

            assert!(w == 1, h == 1);
        }
    }
}