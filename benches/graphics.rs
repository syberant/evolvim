#![cfg_attr(feature = "bench", feature(test))]

#[cfg(all(feature = "bench", test))]
mod benches {
    extern crate gfx;
    extern crate gfx_device_gl;
    extern crate gfx_graphics;
    extern crate graphics;
    extern crate lib_evolvim;
    extern crate shader_version;
    extern crate test;

    use self::gfx::memory::Typed;
    use self::gfx_graphics::*;
    use self::graphics::Context;
    // use self::lib_evolvim::constants::*;
    use self::lib_evolvim::graphics::*;
    // use self::lib_evolvim::*;
    use self::gfx::format::{DepthStencil, Formatted, Srgba8};
    use self::shader_version::OpenGL;
    use self::test::Bencher;

    fn new_view() -> View {
        return View::default();
    }

    // fn new_glyphs(factory: gfx_device_gl::Factory) {
    //     let byte_font = include_bytes!("../src/assets/default-font.ttf");
    //     let text_settings = TextureSettings::new();
    //     let mut glyphs = Glyphs::from_bytes(byte_font, factory, text_settings).unwrap();
    // }

    #[allow(unused_variables, unreachable_code)]
    #[bench]
    fn bench_graphics_complete_draw(b: &mut Bencher) {
        let opengl = OpenGL::V3_2;
        let (device, mut factory) = gfx_device_gl::create(|_s| unimplemented!());

        let aa = 4 as gfx::texture::NumSamples;
        let dim = (1000, 1000, 1, aa.into());
        let color_format = <Srgba8 as Formatted>::get_format();
        let depth_format = <DepthStencil as Formatted>::get_format();
        let (output_color, output_stencil) =
            gfx_device_gl::create_main_targets_raw(dim, color_format.0, depth_format.0);

        let output_color = Typed::new(output_color);
        let output_stencil = Typed::new(output_stencil);
        let viewport = unimplemented!();

        let mut encoder = factory.create_command_buffer().into();
        let mut g2d = Gfx2d::new(opengl, &mut factory);

        // Now initialise variables for evolvim
        let view = new_view();

        b.iter(|| {
            g2d.draw(
                &mut encoder,
                &output_color,
                &output_stencil,
                viewport,
                |context: Context,
                 graphics: &mut GfxGraphics<
                    gfx_device_gl::Resources,
                    gfx_device_gl::CommandBuffer,
                >| {
                    view.board.creatures[0]
                        .borrow()
                        .get_creature()
                        .base
                        .draw(context, graphics, &view);
                },
            );
        });
    }
}
