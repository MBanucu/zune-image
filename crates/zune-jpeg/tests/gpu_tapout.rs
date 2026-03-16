use zune_jpeg::JpegDecoder;
use zune_core::bytestream::ZCursor;

#[test]
fn test_gpu_coefficients_extraction() {
    // This is a placeholder JPEG buffer. Replace with a real JPEG byte array for a real test.
    let jpeg_data = include_bytes!("../../../test-images/jpeg/non_interleaved_420_64x64.jpg");
    let mut decoder = JpegDecoder::new(ZCursor::new(&jpeg_data[..]));
    decoder.gpu_mode = true;
    let _ = decoder.decode();
    let coeffs = decoder.get_gpu_coefficients_and_tables();
    assert!(coeffs.is_some(), "GPU coefficients should be available in gpu_mode");
    let coeffs = coeffs.unwrap();
    // Basic sanity: should have at least one component
    assert!(!coeffs.is_empty(), "Should extract coefficients for at least one component");
    // Each tuple should have a Vec<i16> and a [i32; 64] quantization table
    for (vec, table) in coeffs {
        let vec: Vec<i16> = vec;
        let table: [i32; 64] = table;
        assert_eq!(table.len(), 64);
        assert!(!vec.is_empty());
    }
}
