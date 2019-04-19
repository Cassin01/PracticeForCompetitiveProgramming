from matplotlib import pyplot
import matplotlib 
x = [
      0.001000
    , 0.002000
    , 0.003000 , 0.004000
    , 0.005000
    , 0.006000
    , 0.007000
    , 0.008000
    , 0.009000
    , 0.010000
]

#rand()
y1 = [
      0.000000
    , 0.000020
    , 0.000028
    , 0.000035
    , 0.000050
    , 0.000068
    , 0.000045
    , 0.000090
    , 0.000085
    , 0.000115
]

#MT
y2 = [
      0.000015
    , 0.000040
    , 0.000085
    , 0.000198
    , 0.000278
    , 0.000260
    , 0.000395
    , 0.000500
    , 0.000682
    , 0.000847
]





#pyplot.plot(y1, alpha=0.9, label='steepest_descent', marker='o')
#pyplot.plot(y1, x, alpha=0.9, linestyle='None', label='rand()' , marker='x')
pyplot.plot(y2, x, alpha=0.9, linestyle='None', label='MT' , marker='o')
pyplot.plot(x, x, alpha=0.9, label='Theoretical value')

#pyplot.plot(y3, alpha=0.9, label='quasinewton2' , marker='o')
#pyplot.plot(y3, x, label='quasinewton1')


pyplot.title('Error rate')

pyplot.yscale("log")
pyplot.grid(which="both")

#pyplot.ylim(0, 0.00013)
#pyplot.xlim(0, 0.00012)


pyplot.xlabel('Error rate eps')
pyplot.ylabel('Bit error rate')

#hanrei
pyplot.legend(loc='lower right')

pyplot.savefig("graph1.png")
