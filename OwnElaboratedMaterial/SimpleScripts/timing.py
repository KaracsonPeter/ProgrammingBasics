import time


def get_us():
    this_us = time.time()
    this_s = int(this_us)
    return this_us - this_s


if __name__ == '__main__':
    FLUCTUATION_CENTER = 0.5
    TOLERANCE = 0.01
    while True:
        if (FLUCTUATION_CENTER - TOLERANCE) < get_us() < FLUCTUATION_CENTER:
            break
        time.sleep(0.02)

    TIME_FRAME = 1
    for x in range(0, 50000):
        print(time.time())
        if get_us() < FLUCTUATION_CENTER:
            time.sleep(TIME_FRAME)
        else:
            time.sleep(TIME_FRAME - (2 * TOLERANCE))
