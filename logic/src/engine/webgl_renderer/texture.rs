use image::{codecs::png::PngDecoder, DynamicImage};
use once_cell::unsync::OnceCell;
use web_sys::{WebGlRenderingContext, WebGlTexture};

pub struct Texture {
    png_data: &'static [u8],
    image: OnceCell<DynamicImage>,
    texture: OnceCell<WebGlTexture>,
}

impl Texture {
    pub const fn new(png_data: &'static [u8]) -> Self {
        Self {
            png_data,
            image: OnceCell::new(),
            texture: OnceCell::new(),
        }
    }

    pub fn texture(&self, gl: &WebGlRenderingContext) -> &WebGlTexture {
        if self.texture.get().is_none() {
            let flipped_image = self.image().flipv();
            let pixels = flipped_image.as_rgba8().unwrap().as_flat_samples();

            let texture = gl.create_texture().unwrap();

            gl.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&texture));
            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGBA as i32,
                pixels.layout.width as i32,
                pixels.layout.height as i32,
                0,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(pixels.as_slice()),
            )
            .unwrap();
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_MAG_FILTER,
                WebGlRenderingContext::LINEAR as i32,
            );
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_MIN_FILTER,
                WebGlRenderingContext::LINEAR as i32,
            );
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_WRAP_S,
                WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameteri(
                WebGlRenderingContext::TEXTURE_2D,
                WebGlRenderingContext::TEXTURE_WRAP_T,
                WebGlRenderingContext::CLAMP_TO_EDGE as i32,
            );

            _ = self.texture.set(texture);
        }

        self.texture.get().unwrap()
    }

    pub fn image(&self) -> &DynamicImage {
        if self.image.get().is_none() {
            let image =
                DynamicImage::from_decoder(PngDecoder::new(self.png_data).unwrap()).unwrap();

            _ = self.image.set(image);
        }

        self.image.get().unwrap()
    }
}
