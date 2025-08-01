import time

def main():
    total = 0
    for i in range(1,200_000_000):
        total += i
    
    print(f"The total is {total}")
    return

if __name__ == "__main__":
    start = time.time()
    main()
    finish = time.time() - start
    
    print(f"Total time took is {finish}")