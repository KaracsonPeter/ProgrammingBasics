import time


def get_us():
    this_us = time.time()
    this_s = int(this_us)
    return this_us - this_s


def half_asleep(sleep_for, accuracy):
    """
    This function sleeps the current thread for "sleep_for" seconds within the given accuracy.
    :param sleep_for: Sleep time in seconds.
    :param accuracy: Maximum acceptable error in percentage. Min 0.01; Max 1.0. 1 means 100% error is acceptable.
    """
    start = time.time()
    while True:
        slept_for = time.time() - start
        if slept_for > sleep_for:
            break
        else:
            time.sleep(sleep_for * accuracy)


def catch_fluctuation_center(fluctuation_center, tolerance):
    while True:
        if (fluctuation_center - tolerance) < get_us() < fluctuation_center:
            break
        time.sleep(0.02)


if __name__ == '__main__':
    FLUCTUATION_CENTER = 0.5
    TOLERANCE = 0.03
    TIME_FRAME = 1

    catch_fluctuation_center(FLUCTUATION_CENTER, TOLERANCE)

    for x in range(0, 50000):
        print(time.time())
        # cpu_usage = psutil.cpu_percent(1)  # Calculate CPU usage during one sec
        if get_us() < FLUCTUATION_CENTER:
            half_asleep(TIME_FRAME, TOLERANCE)
        else:
            half_asleep(TIME_FRAME - TOLERANCE, TOLERANCE)
