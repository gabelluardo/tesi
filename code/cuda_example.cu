__global__ void kernelFn(int *vec_a, int *vec_b, int *res, uint n) {
  uint i = threadIdx.x + blockDim.x * blockIdx.x;

  if (i < n) {
    res[i] = vec_a[i] + vec_b[i];
  }
}

int main(int argc, char **argv) {
  uint size = len * sizeof(int);
  int *h_vec_a = reinterpret_cast<int *>(malloc(size));
  int *h_vec_b = reinterpret_cast<int *>(malloc(size));
  int *h_res_vec = reinterpret_cast<int *>(malloc(size));

  int *vec_a, *vec_b, *res_vec;

  cudaMalloc(&vec_a, size);
  cudaMemcpy(vec_a, h_vec_a, size, cudaMemcpyHostToDevice);
  cudaMalloc(&vec_b, size);
  cudaMemcpy(vec_b, h_vec_b, size, cudaMemcpyHostToDevice);

  cudaMalloc(&res_vec, size);

  kernelFn<<<ceil(len / 64.0), 64>>>(vec_a, vec_b, res_vec, len);
  cudaDeviceSynchronize();

  cudaMemcpy(h_res_vec, res_vec, size, cudaMemcpyDeviceToHost);

  // ...

  cudaFree(res_vec);
  cudaFree(vec_b);
  cudaFree(vec_a);
  free(h_res_vec);
  free(h_vec_b);
  free(h_vec_a);

  return 0;
}