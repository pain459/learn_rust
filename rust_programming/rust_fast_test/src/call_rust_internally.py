import subprocess
import time

def sum_of_squares_in_python(n):
    total = 0
    for i in range(n):
        total += i * i
    return total

def sum_of_squares_in_rust(n):
    result = subprocess.run(["./sum_of_squares"], capture_output=True, text=True)
    output = result.stdout.split('\n')
    rust_result = int(output[0].split(': ')[1])
    rust_time = output[1].split(': ')[1]
    return rust_result, rust_time

if __name__ == "__main__":
    n = 10_000_000

    # Run Python calculation
    start_time = time.time()
    python_result = sum_of_squares_in_python(n)
    python_time = time.time() - start_time
    print(f"Python - Sum of squares: {python_result}")
    print(f"Python - Execution time: {python_time} seconds")

    # Run Rust calculation
    rust_result, rust_time = sum_of_squares_in_rust(n)
    print(f"Rust - Sum of squares: {rust_result}")
    print(f"Rust - Execution time: {rust_time}")
