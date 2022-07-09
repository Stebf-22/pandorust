//! Structs for underlying representation of the environment
//!

use image::io::Reader as ImageReader;
use image::GenericImageView;

#[derive(Debug)]
pub struct Matrix<T> {
    data: Vec<T>,
    n_rows: usize,
}

impl<T> Matrix<T> {
    /// Generates a new matrix with shape (n_rows, n_rows)
    ///
    /// # Panics
    /// The function will panic if the data has not the same length as `n_rows`.
    ///
    pub fn new(data: Vec<T>, n_rows: usize) -> Self {
        Matrix { data, n_rows }
    }

    /// Load high-detail BMP file
    pub fn load_image_detail(path: &str) -> Matrix<u16> {
        let image = ImageReader::open(path)
            .expect("No such file")
            .decode()
            .unwrap();

        let img_dims = image.dimensions();

        // Ensure shape
        assert_eq!(img_dims.0, img_dims.1, "Image is not squared");

        Matrix {
            data: image.to_luma16().to_vec(),
            n_rows: img_dims.0 as usize,
        }
    }

    /// Load BMP file
    pub fn load_image(path: &str) -> Matrix<u8> {
        let image = ImageReader::open(path)
            .expect("No such file")
            .decode()
            .unwrap();

        let img_dims = image.dimensions();

        // Ensure shape
        assert_eq!(img_dims.0, img_dims.1, "Image is not squared");

        Matrix {
            data: image.to_luma8().to_vec(),
            n_rows: img_dims.0 as usize,
        }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    /// Generates a Point for a matrix of size `n_rows`
    pub fn new(idx: usize, n_rows: usize) -> Self {
        Point {
            x: idx % n_rows,
            y: idx / n_rows,
        }
    }

    /// Computes the euclidean distance between two points
    ///
    /// NOTE: It is not squared
    ///
    /// A^2 + B^2
    #[inline(always)]
    pub fn euclidean_distance(&self, other: &Point) -> usize {
        (self.x - other.x).pow(2) + (self.y - other.y).pow(2)
    }
}
