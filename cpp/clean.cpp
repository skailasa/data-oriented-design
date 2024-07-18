#include <iostream>
#include <vector>
#include <chrono>
#include <cmath>
#include <cstdlib>

class ShapeBase {
public:
    ShapeBase() {}
    virtual float Area() = 0;
    virtual float Vertices() = 0;
    virtual float VerticesArea()  { return (1.0f / (1.0f + this->Vertices())) * this->Area(); }
    virtual ~ShapeBase() {}
};

class Square : public ShapeBase {
public:
    Square(float SideInit) : Side(SideInit) {}
    virtual float Area() override { return Side * Side; }
    virtual float Vertices() override {return 4.0f; }

private:
    float Side;
};

class Rectangle : public ShapeBase {
public:
    Rectangle(float WidthInit, float HeightInit) : Width(WidthInit), Height(HeightInit) {}
    virtual float Area() override { return Width * Height; }
    virtual float Vertices() override {return 4.0f; }

private:
    float Width, Height;
};

class Triangle : public ShapeBase {
public:
    Triangle(float BaseInit, float HeightInit) : Base(BaseInit), Height(HeightInit) {}
    virtual float Area() override { return 0.5f * Base * Height; }
    virtual float Vertices() override {return 3.0f; }

private:
    float Base, Height;
};


float TotalVerticesAreaVTBL(size_t ShapeCount, ShapeBase **Shapes)
{
    float acc = 0.0f;
    for(size_t idx = 0; idx < ShapeCount; ++idx)
    {
        acc += Shapes[idx] -> VerticesArea();
    }

    return acc;
}

int main() {
    const size_t numShapes = 10000; // Number of shapes for the test
    const size_t numIterations = 1000; // Number of iterations for averaging

    std::vector<ShapeBase*> shapes;
    for (size_t i = 0; i < numShapes; ++i) {
        shapes.push_back(new Square(5.0f));
        shapes.push_back(new Rectangle(4.0f, 6.0f));
        shapes.push_back(new Triangle(3.0f, 7.0f));
    }

    using namespace std::chrono;
    auto start = high_resolution_clock::now();

    for (size_t i = 0; i < numIterations; ++i) {
        // TotalAreaVTBL(shapes.size(), shapes.data());
        TotalVerticesAreaVTBL(shapes.size(), shapes.data());
    }

    auto end = high_resolution_clock::now();
    duration<double> elapsed = end - start;

    double avgTimePerIteration = elapsed.count() / numIterations;
    double avgTimePerShape = avgTimePerIteration / numShapes ;

    // Assuming your CPU has a clock speed of 3 GHz (3 * 10^9 cycles per second)
    double cpuClockSpeedGHz = 3.2;
    double cyclesPerShape = avgTimePerShape * cpuClockSpeedGHz * 1e9;

    std::cout << "(C++/OOP) Average cycles per shape: " << cyclesPerShape << std::endl;

    // Clean up
    for (auto shape : shapes) {
        delete shape;
    }

    return 0;
}