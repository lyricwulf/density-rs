use density_sys as sys;

#[repr(i32)]
pub enum Algorithm {
    Chameleon = sys::DENSITY_ALGORITHM_DENSITY_ALGORITHM_CHAMELEON,
    Cheetah = sys::DENSITY_ALGORITHM_DENSITY_ALGORITHM_CHEETAH,
    Lion = sys::DENSITY_ALGORITHM_DENSITY_ALGORITHM_LION,
}

pub fn compress_safe_size(text_length: u64) -> u64 {
    unsafe { sys::density_compress_safe_size(text_length) }
}

pub fn decompress_safe_size(text_length: u64) -> u64 {
    unsafe { sys::density_decompress_safe_size(text_length) }
}

pub fn compress_safe_size_slice<T>(text_length: &[T]) -> u64 {
    compress_safe_size(std::mem::size_of_val(text_length) as u64)
}

pub fn decompress_safe_size_slice<T>(text_length: &[T]) -> u64 {
    decompress_safe_size(std::mem::size_of_val(text_length) as u64)
}

pub fn density_compress(
    input: &[u8],
    output: &mut [u8],
    algorithm: Algorithm,
) -> sys::density_processing_result {
    unsafe {
        sys::density_compress(
            input.as_ptr(),
            input.len() as _,
            output.as_mut_ptr(),
            output.len() as _,
            algorithm as _,
        )
    }
}

pub fn density_decompress(input: &[u8], output: &mut [u8]) -> sys::density_processing_result {
    unsafe {
        sys::density_decompress(
            input.as_ptr(),
            input.len() as _,
            output.as_mut_ptr(),
            output.len() as _,
        )
    }
}

pub fn compress(input: &[u8], algorithm: Algorithm) -> Vec<u8> {
    let mut output = vec![0u8; compress_safe_size_slice(input) as _];
    let result = density_compress(input, &mut output, algorithm);
    unsafe {
        output.set_len(result.bytesWritten as _);
    }
    output
}

pub fn decompress(input: &[u8]) -> Vec<u8> {
    let mut output = vec![0u8; decompress_safe_size_slice(input) as _];
    let result = density_decompress(input, &mut output);
    unsafe {
        output.set_len(result.bytesWritten as _);
    }
    output
}
