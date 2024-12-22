export async function deflateRawCompress(data: Uint8Array): Promise<Uint8Array> {
  const cs = new CompressionStream("deflate-raw");
  const writer = cs.writable.getWriter();
  await writer.write(data);
  await writer.close();

  const buffer = await new Response(cs.readable).arrayBuffer();
  return new Uint8Array(buffer);
}

export async function deflateRawDecompress(data: Uint8Array): Promise<Uint8Array> {
  const inputStream = new ReadableStream({
    start(controller) {
      controller.enqueue(data);
      controller.close();
    },
  });

  const ds = new DecompressionStream("deflate-raw");
  const decompressedStream = inputStream.pipeThrough(ds);
  const buffer = await new Response(decompressedStream).arrayBuffer();

  return new Uint8Array(buffer);
}
