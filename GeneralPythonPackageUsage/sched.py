import sched
import time


def print_time(parameter='default argument'):
    print("From print_time", time.time(), parameter)


s = sched.scheduler(time.time, time.sleep)

if __name__ == '__main__':
    """
    Example for using sched package. 
    1. argument: You can set, with how much delay the passed function should be called.
    2.: Priority of passed of the function in case of simultaneous call with other functions.
    3.: function name
    4.: arguments of passed function either as positional or keyword arg.
    """
    HIGHEST_PRIORITY = 1
    MIDDLE_PRIORITY = 2
    LOWEST_PRIORITY = 3

    DELAY_1 = 1.000001  # delay in us precision
    s.enter(DELAY_1, LOWEST_PRIORITY, print_time)
    s.enter(DELAY_1, HIGHEST_PRIORITY, print_time, argument=('positional argument',))
    s.enter(DELAY_1, MIDDLE_PRIORITY, print_time, kwargs={'parameter': 'keyword argument'})
    s.run()

    # !!! It wont run in each second! Execution of print takes some time.
    for x in range(0, 9):
        s.enter(DELAY_1, LOWEST_PRIORITY, print_time)
        s.run()