from matplotlib import pyplot
import matplotlib

x = [
      0.000010
    , 0.000020
    , 0.000030
    , 0.000040
    , 0.000050
    , 0.000060
    , 0.000070
    , 0.000080
    , 0.000090
    , 0.000100
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
     0.000011
    ,0.000021
    ,0.000032
    ,0.000041
    ,0.000048
    ,0.000052
    ,0.000069
    ,0.000080
    ,0.000091
    ,0.000104
]





#pyplot.plot(y1, alpha=0.9, label='steepest_descent', marker='o')
pyplot.plot(y1, x, alpha=0.9, linestyle='None', label='rand()' , marker='x')
pyplot.plot(y2, x, alpha=0.9, linestyle='None', label='MT' , marker='o')
pyplot.plot(x, x, alpha=0.9, label='Theoretical value')

#pyplot.plot(y3, alpha=0.9, label='quasinewton2' , marker='o')
#pyplot.plot(y3, x, label='quasinewton1')


pyplot.title('Error rate')

pyplot.yscale("log")
pyplot.grid(which="both")

pyplot.ylim(0, 0.00013)
pyplot.xlim(0, 0.00012)


pyplot.xlabel('Error rate eps')
pyplot.ylabel('Bit error rate')

#hanrei
pyplot.legend(loc='lower right')

pyplot.savefig("graph1.png")
