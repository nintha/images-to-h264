use {Colorspace, Encoding, Modifier};
use core::marker::PhantomData;
use core::ptr;
use x264::*;

/// Input image data to be given to the encoder.
pub struct Image<'a> {
    raw: x264_image_t,
    width: i32,
    height: i32,
    spooky: PhantomData<&'a [u8]>,
}

impl<'a> Image<'a> {
    /// Makes a new image with the given information.
    ///
    /// # Panics
    ///
    /// Panics if the plane is invalid.
    pub fn new<E: Into<Encoding>>(
        format: E,
        width: i32,
        height: i32,
        planes: &[Plane<'a>],
    ) -> Self {
        //TODO: Get someone who knows what they're doing to verify this.

        use self::Colorspace::*;

        let format = format.into();

        // pc: planes count
        let (pc, wm, hm, ws, hs): (_, _, _, &[_], &[_]) =
            match format.colorspace() {
                I420 | YV12 => (3, 2, 2, &[2, 1, 1], &[2, 1, 1]),
                NV12 | NV21 => (2, 2, 2, &[2, 2], &[2, 1]),
                I422 | YV16 => (3, 2, 1, &[2, 1, 1], &[1, 1, 1]),
                NV16 => (2, 2, 1, &[2, 2], &[1, 1]),
                #[cfg(feature = "yuyv")]
                YUYV | UYVY => (1, 1, 1, &[2], &[1]),
                V210 => (1, 1, 1, &[4], &[1]),
                I444 | YV24 => (3, 1, 1, &[1, 1, 1], &[1, 1, 1]),
                BGR | RGB => (1, 1, 1, &[3], &[1]),
                BGRA => (1, 1, 1, &[4], &[1]),
            };

        let (wq, wr) = (width / wm, width % wm);
        let (hq, hr) = (height / hm, height % hm);
        let depth = if format.has(Modifier::HighDepth) { 2 } else { 1 };

        // Check that the number of planes matches pc.
        assert!(planes.len() == pc);
        // Check that the width and the height are multiples of wm and hm.
        assert!(wr == 0 && hr == 0);
        for (i, plane) in planes.iter().enumerate() {
            // Check that the plane's stride is at least depth * wq * ws[i].
            assert!(depth * wq * ws[i] <= plane.stride);
            // Check that there are at least hq * hs[i] rows in the plane.
            assert!(
                hq * hs[i] <= plane.data.len() as i32 / plane.stride,
                "hq * hs[i] <= plane.data.len() as i32 / plane.stride, left={}, right={}, plane_index={}",
                hq * hs[i],
                plane.data.len() as i32 / plane.stride,
                i,
            );
        }

        unsafe {
            Self::new_unchecked(format, width, height, planes)
        }
    }

    /// Makes a new packed BGR image.
    pub fn bgr(width: i32, height: i32, data: &'a [u8]) -> Self {
        let plane = Plane { stride: data.len() as i32 / height, data };
        Self::new(Colorspace::BGR, width, height, &[plane])
    }

    /// Makes a new packed RGB image.
    pub fn rgb(width: i32, height: i32, data: &'a [u8]) -> Self {
        let plane = Plane { stride: data.len() as i32 / height, data };
        Self::new(Colorspace::RGB, width, height, &[plane])
    }

    /// Makes a new packed BGRA image.
    pub fn bgra(width: i32, height: i32, data: &'a [u8]) -> Self {
        let plane = Plane { stride: data.len() as i32 / height, data };
        Self::new(Colorspace::BGRA, width, height, &[plane])
    }
    /// Makes a new packed YUV420p image.
    pub fn yuv420p(width: i32, height: i32, data: &'a [u8]) -> Self {
        let one_six = data.len() / 6;
        let plane_y = Plane { stride: one_six as i32 * 4 / height, data: &data[0..one_six * 4] };
        let plane_u = Plane { stride: one_six as i32 * 2 / height, data: &data[one_six * 4..one_six * 5] };
        let plane_v = Plane { stride: one_six as i32 * 2 / height, data: &data[one_six * 5..one_six * 6] };
        Self::new(Colorspace::I420, width, height, &[plane_y, plane_u, plane_v])
    }

    /// Makes a new image with the given planes and colorspace.
    ///
    /// # Unsafety
    ///
    /// The caller must ensure that the plane fulfils all the invariants that
    /// x264 expects it to fulfil. I don't actually know what all of those are,
    /// but the source of `Encoder::new` is my best guess.
    pub unsafe fn new_unchecked(
        format: Encoding,
        width: i32,
        height: i32,
        planes: &[Plane<'a>],
    ) -> Self {
        let mut strides = [0; 4];
        let mut pointers = [ptr::null_mut(); 4];

        for (i, &Plane { stride, data }) in planes.iter().enumerate() {
            strides[i] = stride;
            pointers[i] = data.as_ptr() as *mut u8;
        }

        let raw = x264_image_t {
            i_csp: format.into_raw(),
            i_plane: planes.len() as i32,
            i_stride: strides,
            plane: pointers,
        };

        Self { raw, width, height, spooky: PhantomData }
    }

    // Getters

    /// The width of the image.
    pub fn width(&self) -> i32 { self.width }
    /// The height of the image.
    pub fn height(&self) -> i32 { self.height }
    /// The encoding of the image.
    pub fn encoding(&self) -> Encoding {
        unsafe { Encoding::from_raw(self.raw.i_csp) }
    }

    #[doc(hidden)]
    pub fn raw(&self) -> x264_image_t { self.raw }
}

/// A single plane of an image.
pub struct Plane<'a> {
    /// The plane's stride (the number of bytes for each row).
    pub stride: i32,
    /// The plane's pixel data.
    pub data: &'a [u8],
}
