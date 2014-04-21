use std::libc;
use std::ptr;
use std::io::File;

pub struct CUctx_st
{
    id : u64
}
pub struct CUfunc_st
{
    id : u64
}
pub struct CUmod_st
{
    id : u64
}
pub struct CUstream_st
{
    id : u64
}

pub type CUdeviceptr = libc::c_uint;
pub type CUerror = i32;

#[link(name = "cuda")]
extern{
    pub fn cuInit(flags :u64) -> CUerror;
    pub fn cuDeviceGetCount(count : *i32) -> CUerror;
    pub fn cuDeviceGet(device : *i32, ordinal : i32) -> CUerror;
    pub fn cuDeviceGetName(name : *mut i8, len : i32, device : i32) -> CUerror;
    pub fn cuDeviceComputeCapability(major: *mut i32, minor: *mut i32, dev : i32 ) -> CUerror;
    pub fn cuCtxCreate(pctx: *mut CUctx_st, flags: u64, dev : i32) -> CUerror;
    pub fn cuModuleGetFunction(hfunc : *mut *CUfunc_st ,hmod: *CUmod_st, name: *i8) -> CUerror;
    pub fn cuModuleLoadDataEx(module : *mut *CUmod_st, image: *i8, numOptions: libc::c_uint, options: *i32, optionsValues : *i8) -> CUerror;
    pub fn cuMemAlloc(dptr: *mut CUdeviceptr, bytesize: libc::c_uint) -> CUerror;
    pub fn cuMemcpyHtoD(dstDevice: CUdeviceptr, hostData: *libc::c_void, byteCount: libc::size_t) -> CUerror;
    pub fn cuLaunchKernel(f: *CUfunc_st, gridDimX: libc::c_uint, gridDimY: libc::c_uint, gridDimZ: libc::c_int, 
                          blockDimX: libc::c_uint, blockDimY: libc::c_uint, blockDimZ: libc::c_uint, 
                          sharedMembytes: libc::c_uint, hStream: *libc::c_void, kernelParams: **libc::c_void, extra: **libc::c_void) -> CUerror;

    pub fn cuMemcpyDtoH(dstHost: *libc::c_void, dstDevice: CUdeviceptr, bytecount: libc::size_t) -> CUerror;
}

fn checkCudaErrors(errorNum: CUerror) {
    if errorNum != 0 {
        println!("CUDA error. Num: {}", errorNum);
        unsafe { libc::exit(1 as libc::c_int); }
    }
}

fn main(){
    unsafe {
        let devcount = 0;
        let device = 0;
        let mut name= Vec::with_capacity(128) ;
        let mut devMajor = 0;
        let mut devMinor = 0;
        let mut context = ~CUctx_st{id : 0};
        checkCudaErrors(cuInit(0));
        println!("{}", 1);
        checkCudaErrors(cuDeviceGetCount(&devcount));
        println!("{}", 2);
        checkCudaErrors(cuDeviceGet(&device, 0));
        println!("{}", 3);
        checkCudaErrors(cuDeviceGetName(name.as_mut_ptr(),128, device));
        println!("{}", 4);
        checkCudaErrors(cuDeviceComputeCapability(&mut devMajor, &mut devMinor, device));
        println!("{}", 5);
        checkCudaErrors(cuCtxCreate(&mut *context, 0,  device));
        println!("{}", 6);


        let ptxdata = File::open(&Path::new("shared.ptx")).read_to_end().unwrap();
        let kernel = "kernel";
        let mut function = ptr::null();
        let mut cudamodule = ptr::null();
        checkCudaErrors(cuModuleLoadDataEx(&mut cudamodule, ptxdata.to_c_str().unwrap(), 0, ptr::null(), ptr::null()));
        println!("{}", 7);
        checkCudaErrors(cuModuleGetFunction(&mut function, cudamodule, kernel.to_c_str().unwrap()));
        println!("{}", 8);

        let mut nums_dev = 0 as CUdeviceptr;
        let mut counts_dev = 0 as CUdeviceptr;
        checkCudaErrors(cuMemAlloc(&mut nums_dev, (std::mem::size_of::<i32>() * 16) as u32));
        println!("{}", 9);
        checkCudaErrors(cuMemAlloc(&mut counts_dev, (std::mem::size_of::<i32>() * 16) as u32));
        println!("{}", 10);

        let mut KernelParams = [&nums_dev, &counts_dev];
        let values = range(1,9);
        let mut hostNums = Vec::from_fn(16, |i| i/2);
        let mut blockSizeX = 16;
        let mut blockSizeY = 1;
        let mut blockSizeZ = 1;
        let mut gridSizeX = 1;
        let mut gridSizeY = 1;
        let mut gridSizeZ = 1;

        checkCudaErrors(cuMemcpyHtoD(nums_dev, hostNums.as_ptr() as *libc::c_void, (std::mem::size_of::<i32>() * 16) as u64));
        checkCudaErrors(cuLaunchKernel(function, gridSizeX, gridSizeY, gridSizeZ,
        blockSizeX, blockSizeY, blockSizeZ,
        0, ptr::null(), KernelParams.as_ptr() as **libc::c_void, ptr::null()));
        println!("{}", 11);

        let mut hostcounts: Vec<i32> = Vec::with_capacity(16);
        checkCudaErrors(cuMemcpyDtoH(hostcounts.as_ptr() as *libc::c_void, counts_dev, (std::mem::size_of::<i32>() * 16) as u64));
        println!("{}", 12);
    }
}
