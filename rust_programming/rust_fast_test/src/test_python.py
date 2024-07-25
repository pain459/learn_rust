import time

def sum_of_squares(n):
    total = 0
    for i in range(n):
        total += i * i
    return total

if __name__ == "__main__":
    n = 10_000_000_000
    start_time = time.time()
    result = sum_of_squares(n)
    end_time = time.time()
    print(f"Sum of squares: {result}")
    print(f"Execution time: {end_time - start_time} seconds")
