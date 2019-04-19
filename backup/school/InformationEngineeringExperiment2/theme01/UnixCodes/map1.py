from matplotlib import pyplot
import matplotlib



x = [x for x in range(40)]

# steepest_descent
y1 = [
      8257.58
    , 1151.2
    , 186.851
    , 120.047
    , 46.5493
    , 6.8475
    , -0.944785
    , -2.24508
    , -2.37207
    , -2.39764
    , -2.40419
    , -2.40724
    , -2.40779
    , -2.40787
    , -2.40787
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    ]

y2 = [
      8257.58
    , 1.83E+09
    , 5.92E+11
    , 1.78E+14
    , 5.10E+16
    , 1.41E+19
    , 3.82E+21
    , 1.01E+24
    , 2.62E+26
    , 6.61E+28
    , 1.28E+31
]

y3 = [
      8257.58
    , 1151.2
    , 939.18
    , 503.573
    , 347.131
    , 247.134
    , 194.932
    , 163.962
    , 144.425
    , 129.197
    , 112.532
    , 84.8035
    , 24.1707
    , 6.27607
    , 2.02866
    , -0.249391
    , -1.63558
    , -2.27248
    , -2.35342
    , -2.39938
    , -2.40489
    , -2.40659
    , -2.40784
    , -2.40787
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
    , -2.40788
]


pyplot.ylim(-10, 8400)
pyplot.xlim(0, 40)

pyplot.plot(y1, alpha=0.9, label='steepest_descent', marker='o')
pyplot.plot(y2, alpha=0.9, label='newton1'         , marker='x')
pyplot.plot(y3, alpha=0.9, label='quasinewton1'         )
#pyplot.plot(y3, x, label='quasinewton1')

pyplot.title('Unlimited optimization')



pyplot.xlabel('X-Axis')
pyplot.ylabel('Y-Axis')

#グラフの凡例
pyplot.legend()

pyplot.savefig("graph1.png")
