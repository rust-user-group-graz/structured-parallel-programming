#include "TestMandelbrot.h"
#include "tbb/parallel_for.h"
#include "tbb/blocked_range.h"

using namespace tbb;

static constexpr size_t IMAGE_WIDTH = 1024;
static constexpr size_t IMAGE_HEIGHT = 1024;

static constexpr double X_START = -1.0;
static constexpr double X_END =    0.5;

static constexpr double Y_START = -0.7;
static constexpr double Y_END =    0.7;

static constexpr double X_STEP = (X_END - X_START)/IMAGE_WIDTH;
static constexpr double Y_STEP = (Y_END - Y_START)/IMAGE_HEIGHT;

static constexpr int MAX_ITER = 127;

int mandelbrot(double start_real, double start_imag)
{
    double z_real = start_real;
    double z_imag = start_imag;

    for (int n = 0; n < MAX_ITER; ++n)
    {
        double r2 = z_real * z_real;
        double i2 = z_imag * z_imag;
        if (r2 + i2 > 4.0) {
            return n;
        }
        z_imag = 2.0 * z_real * z_imag + start_imag;
        z_real = r2 - i2 + start_real;
    }
    return MAX_ITER;
}

struct Mandelbrot {
    int* framebuffer;

    void operator()( const blocked_range<int>& range ) const {
        for( int i = range.begin(); i != range.end(); ++i )
        {
            framebuffer[i] = 0;
        }
    }
};

void TestMandelbrot::testMandelbrot() {
    QBENCHMARK {
        int framebuffer[IMAGE_WIDTH * IMAGE_HEIGHT];
        Mandelbrot mandelbrot;
        mandelbrot.framebuffer = framebuffer;
        parallel_for( blocked_range<int>( 1, IMAGE_WIDTH * IMAGE_HEIGHT ), mandelbrot );
    };
}