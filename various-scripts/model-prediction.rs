use tract_onnx::prelude::*;
use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
//`RUST_BACKTRACE=1`

fn main() -> TractResult<()> {
    let model = tract_onnx::onnx()
        .model_for_path("/path/EFN-color-new.onnx")?
        .with_output_fact(0, Default::default())?
        .into_optimized()?
        .into_runnable()?;
    
    //println!("hello");

    // open image, resize it and make a Tensor out of it
    let image = image::open("/path/blue-car.jpg").unwrap().to_rgb8();

    let (width, height) = image.dimensions();
    println!("Width {} and height {}", width, height);
    
    let resized =
        image::imageops::resize(&image, 224, 224, ::image::imageops::FilterType::Triangle);
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((224, 224, 3, 1), |(_, c, y, x)| {
        let mean = [0.485, 0.456, 0.406][c];
        let std = [0.229, 0.224, 0.225][c];
        (resized[(x as _, y as _)][c] as f32 / 255.0 - mean) / std
    })
    .into();
    
    // run the model on the input
    let result = model.run(tvec!(image.into()))?;

    // find and display the max value with its index
    let best = result[0]
        .to_array_view::<f32>()?
        .iter()
        .cloned()
        .zip(2..)
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
    println!("result: {best:?}");
    
    Ok(())
}

