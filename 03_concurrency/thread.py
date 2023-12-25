from threading import Thread
from multiprocessing import Process
from time import sleep
from typing import Optional
import time


# class ExampleThread(Thread):
#     def __init__(self, seconds: int, name: str) -> None:
#         super().__init__()
#         self.seconds: int = seconds
#         self.name: str = name
#         self._return: Optional[int] = None
#
#     def run(self) -> None:
#         print(f"thread {self.name} is running")
#         sleep(self.seconds)
#         print(f"thread {self.name} has finished")
#         self._return = self.seconds
#
#     def join(self) -> int:
#         Thread.join(self)
#         return self._return
#
#
# class ExampleProcess(Process):
#     def __init__(self, seconds: int, name: str) -> None:
#         super().__init__()
#         self.seconds: int = seconds
#         self.name: str = name
#         self._return: Optional[str] = None
#
#     def run(self) -> None:
#         # do something bro
#         pass
#
#     def join(self) -> int:
#         Process.join(self)
#         return self._return
#
#
# one: ExampleThread = ExampleThread(seconds=1, name="one")
# two: ExampleThread = ExampleThread(seconds=1, name="two")
# three: ExampleThread = ExampleThread(seconds=1, name="three")
# # breakpoint()
#
# start = time.time()
# one.start()
# two.start()
# three.start()
#
# print("started all threads")
# one_res = one.join()
# two_res = two.join()
# three_res = three.join()
#
# finish = time.time()
# print(f"{finish-start} has elapsed")
#
# print(one_res)
# print(two_res)
# print(three_res)


if __name__ == "__main__":

    def recur_fibo(n: int) -> int:
        if n <= 1:
            return n
        else:
            return recur_fibo(n - 1) + recur_fibo(n - 2)

    start = time.time()
    recur_fibo(n=8)
    recur_fibo(n=12)
    recur_fibo(n=12)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=20)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=28)
    recur_fibo(n=36)
    finish = time.time()
    print(f"{finish - start} has elapsed for this stupid fibo test")

    from multiprocessing import Pool

    start = time.time()
    with Pool(4) as p:
        print(
            p.starmap(
                recur_fibo,
                [
                    (8,),
                    (12,),
                    (12,),
                    (20,),
                    (20,),
                    (20,),
                    (20,),
                    (28,),
                    (28,),
                    (28,),
                    (28,),
                    (36,),
                    # (21,),
                    # (144,),
                    # (144,),
                    # (6765,),
                    # (6765,),
                    # (6765,),
                    # (6765,),
                    # (317811,),
                    # (317811,),
                    # (317811,),
                    # (317811,),
                    # (14930352,),
                    # (1836311903,),
                    # (1836311903,),
                    # (1836311903),
                ],
            )
        )
    finish = time.time()
    print(f"{finish - start} has elapsed for this stupid fibo test using pool")
