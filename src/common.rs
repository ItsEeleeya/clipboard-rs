use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView, ImageFormat};
use std::error::Error;
use std::io::Cursor;

/// A type alias for a result that can return any type of error.
pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync + 'static>>;

/// A trait for clipboard content data.
pub trait ContentData {
	/// Get the format of the content.
	fn get_format(&self) -> ContentFormat;

	/// Get the content as a byte slice.
	fn as_bytes(&self) -> &[u8];

	/// Get the content as a string.
	fn as_str(&self) -> Result<&str>;
}

/// A trait for clipboard handlers.
pub trait ClipboardHandler {
	/// Called when the clipboard content changes.
	fn on_clipboard_change(&mut self);
}

/// An enum representing different types of clipboard content.
pub enum ClipboardContent {
	Text(String),
	Rtf(String),
	Html(String),
	Image(RustImageData),
	Files(Vec<String>),
	Other(String, Vec<u8>),
}

impl ContentData for ClipboardContent {
	fn get_format(&self) -> ContentFormat {
		match self {
			ClipboardContent::Text(_) => ContentFormat::Text,
			ClipboardContent::Rtf(_) => ContentFormat::Rtf,
			ClipboardContent::Html(_) => ContentFormat::Html,
			ClipboardContent::Image(_) => ContentFormat::Image,
			ClipboardContent::Files(_) => ContentFormat::Files,
			ClipboardContent::Other(format, _) => ContentFormat::Other(format.clone()),
		}
	}

	fn as_bytes(&self) -> &[u8] {
		match self {
			ClipboardContent::Text(data) => data.as_bytes(),
			ClipboardContent::Rtf(data) => data.as_bytes(),
			ClipboardContent::Html(data) => data.as_bytes(),
			ClipboardContent::Image(_) => &[],
			ClipboardContent::Files(data) => {
				if let Some(path) = data.first() {
					path.as_bytes()
				} else {
					&[]
				}
			}
			ClipboardContent::Other(_, data) => data.as_slice(),
		}
	}

	fn as_str(&self) -> Result<&str> {
		match self {
			ClipboardContent::Text(data) => Ok(data),
			ClipboardContent::Rtf(data) => Ok(data),
			ClipboardContent::Html(data) => Ok(data),
			ClipboardContent::Image(_) => Err("can't convert image to string".into()),
			ClipboardContent::Files(data) => {
				if let Some(path) = data.first() {
					Ok(path)
				} else {
					Err("content is empty".into())
				}
			}
			ClipboardContent::Other(_, data) => std::str::from_utf8(data).map_err(|e| e.into()),
		}
	}
}

/// An enum representing different formats of clipboard content.
#[derive(Clone)]
pub enum ContentFormat {
	Text,
	Rtf,
	Html,
	Image,
	Files,
	Other(String),
}

/// A struct representing image data in Rust.
pub struct RustImageData {
	width: u32,
	height: u32,
	data: Option<DynamicImage>,
}

/// A struct representing image data in Rust as a byte buffer.
pub struct RustImageBuffer(Vec<u8>);

/// A trait for manipulating images in Rust.
pub trait RustImage: Sized {
	/// Create an empty image.
	fn empty() -> Self;

	/// Check if the image is empty.
	fn is_empty(&self) -> bool;

	/// Read an image from a file path.
	fn from_path(path: &str) -> Result<Self>;

	/// Create a new image from a byte slice.
	fn from_bytes(bytes: &[u8]) -> Result<Self>;

	/// Create a new image from a dynamic image.
	fn from_dynamic_image(image: DynamicImage) -> Self;

	/// Get the size (width and height) of the image.
	fn get_size(&self) -> (u32, u32);

	/// Scale the image down to fit within a specific size.
	fn thumbnail(&self, width: u32, height: u32) -> Result<Self>;

	/// Resize the image to a specific size.
	fn resize(&self, width: u32, height: u32, filter: FilterType) -> Result<Self>;

	/// Convert the image to JPEG format.
	fn to_jpeg(&self) -> Result<RustImageBuffer>;

	/// Convert the image to PNG format.
	fn to_png(&self) -> Result<RustImageBuffer>;

	/// Convert the image to bitmap format.
	fn to_bitmap(&self) -> Result<RustImageBuffer>;

	/// Save the image to a file path.
	fn save_to_path(&self, path: &str) -> Result<()>;
}

macro_rules! image_to_format {
	($name:ident, $format:expr) => {
		fn $name(&self) -> Result<RustImageBuffer> {
			match &self.data {
				Some(image) => {
					let mut bytes: Vec<u8> = Vec::new();
					image.write_to(&mut Cursor::new(&mut bytes), $format)?;
					Ok(RustImageBuffer(bytes))
				}
				None => Err("image is empty".into()),
			}
		}
	};
}

impl RustImage for RustImageData {
	fn empty() -> Self {
		RustImageData {
			width: 0,
			height: 0,
			data: None,
		}
	}

	fn is_empty(&self) -> bool {
		self.data.is_none()
	}

	fn from_path(path: &str) -> Result<Self> {
		let image = image::open(path)?;
		let (width, height) = image.dimensions();
		Ok(RustImageData {
			width,
			height,
			data: Some(image),
		})
	}

	fn from_bytes(bytes: &[u8]) -> Result<Self> {
		let image = image::load_from_memory(bytes)?;
		let (width, height) = image.dimensions();
		Ok(RustImageData {
			width,
			height,
			data: Some(image),
		})
	}

	fn from_dynamic_image(image: DynamicImage) -> Self {
		let (width, height) = image.dimensions();
		RustImageData {
			width,
			height,
			data: Some(image),
		}
	}

	fn get_size(&self) -> (u32, u32) {
		(self.width, self.height)
	}

	fn thumbnail(&self, width: u32, height: u32) -> Result<Self> {
		match &self.data {
			Some(image) => {
				let resized = image.thumbnail(width, height);
				Ok(RustImageData {
					width: resized.width(),
					height: resized.height(),
					data: Some(resized),
				})
			}
			None => Err("image is empty".into()),
		}
	}

	fn resize(&self, width: u32, height: u32, filter: FilterType) -> Result<Self> {
		match &self.data {
			Some(image) => {
				let resized = image.resize_exact(width, height, filter);
				Ok(RustImageData {
					width: resized.width(),
					height: resized.height(),
					data: Some(resized),
				})
			}
			None => Err("image is empty".into()),
		}
	}

	image_to_format!(to_jpeg, ImageFormat::Jpeg);

	image_to_format!(to_png, ImageFormat::Png);

	image_to_format!(to_bitmap, ImageFormat::Bmp);

	fn save_to_path(&self, path: &str) -> Result<()> {
		match &self.data {
			Some(image) => {
				image.save(path)?;
				Ok(())
			}
			None => Err("image is empty".into()),
		}
	}
}

impl RustImageBuffer {
	/// Get the byte slice of the image buffer.
	pub fn get_bytes(&self) -> &[u8] {
		&self.0
	}

	/// Save the image buffer to a file path.
	pub fn save_to_path(&self, path: &str) -> Result<()> {
		std::fs::write(path, &self.0)?;
		Ok(())
	}
}
