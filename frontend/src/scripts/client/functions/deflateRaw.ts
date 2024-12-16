export async function deflateRawCompress(data: Uint8Array): Promise<Uint8Array> {
  const cs = new CompressionStream("deflate-raw");
  const writer = cs.writable.getWriter();
  await writer.write(data);
  await writer.close();

  const buffer = await new Response(cs.readable).arrayBuffer();
  return new Uint8Array(buffer);
}

export async function deflateRawDecompress(data: Uint8Array): Promise<Uint8Array> {
  try {
    console.log("Starting decompression, input size:", data.length);

    // Create a ReadableStream from the input data
    const inputStream = new ReadableStream({
      start(controller) {
        controller.enqueue(data);
        controller.close();
      },
    });

    // Create the DecompressionStream for "deflate-raw"
    const ds = new DecompressionStream("deflate-raw");
    console.log("Created DecompressionStream");

    // Pipe the input stream through the decompression stream
    const decompressedStream = inputStream.pipeThrough(ds);
    console.log("Piped input stream into DecompressionStream");

    // Convert the decompressed stream into an ArrayBuffer
    const buffer = await new Response(decompressedStream).arrayBuffer();
    console.log("Converted decompressed stream to ArrayBuffer");

    // Return as Uint8Array
    return new Uint8Array(buffer);
  } catch (e) {
    console.error("Error in deflateRawDecompress:", e);
    throw e; // Re-throw to be caught by caller
  }
}
