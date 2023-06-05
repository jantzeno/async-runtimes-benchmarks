package main

import "math"

type runningStat struct {
	count   int
	oldMean float64
	newMean float64
	oldSum  float64
	newSum  float64
}

func (rs *runningStat) clear() {
	rs.count = 0
	rs.oldMean = 0
	rs.newMean = 0
	rs.oldSum = 0
	rs.newSum = 0
}

func (rs *runningStat) mean() float64 {
	if rs.count > 0 {
		return rs.newMean
	} else {
		return 0
	}
}

func (rs *runningStat) variance() float64 {
	if rs.count > 1 {
		return rs.newSum / float64(rs.count-1)
	} else {
		return 0
	}
}

func (rs *runningStat) standardDeviation() float64 {
	return math.Sqrt(rs.variance())
}

func (rs *runningStat) push(value float64) {
	rs.count++

	if rs.count == 1 {
		rs.oldMean = value
		rs.newMean = value
		rs.oldSum = 0.0
	} else {
		rs.newMean = rs.oldMean + (value-rs.oldMean)/float64(rs.count)
		rs.newSum = rs.oldSum + (value-rs.oldMean)*(value-rs.newMean)

		// set up for next iteration
		rs.oldMean = rs.newMean
		rs.oldSum = rs.newSum
	}
}
