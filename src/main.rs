use image::{io::Reader as ImageReader, ImageError, GenericImageView, Pixel, Rgba, ImageBuffer, RgbaImage};

fn main() -> Result<(), ImageError>{
	let path = "test-data/brick.jpg";

	let original = ImageReader::open(path)?.decode()?;

	let pixels = original.pixels();

	let width = original.width();
	let height = original.height();
	let mut buf: RgbaImage = ImageBuffer::new(width, height);

	pixels.for_each(|(x, y, color)| {
		let colors = color.channels();

		let r = colors[0];
		let g = colors[1];
		let b = colors[2];
		let a = colors[3];

		let avg = r/3 + g/3 + b/3;

		buf.put_pixel(x, y, Rgba([avg, avg, avg, a]));
	});

	buf.save("test.png")?;

	Ok(())
}
