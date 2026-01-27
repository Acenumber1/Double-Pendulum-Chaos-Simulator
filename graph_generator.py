#for making the image from the array
from PIL import Image
#for math
import numpy as np
#lets multiple loops threads run in parallel
from numba import njit, prange
#for runtime
import time


graphRes = 1024 #larger of the two resolutions
totalTime = 25 # maximum simulation time per pixel
center1 = 0 #center of range for theta1 [-1,1]
center2 = 0  #center of range for theta2 [-1,1]
range1 = 1 #width of range for theta1 [0,1]
range2 = 1 #width of range for theta2 [0,1]
dt = 0.01 # timestep
do_printout = True
g = 9.81 # gravity
safe_color = (255,255,255) #color of non-chaotic pendulums

#1 is the y-axis, 2 is the x-axis
#note that the bottom left is -180,-180 and the top right is 180,180
#angle range for the first pendulum (max is [-180,180]
start1 = (center1-range1)*np.pi
end1 = (center1+range1)*np.pi
#angle range for the second pendulum (max is [-180,180]
start2 = (center2-range2)*np.pi
end2 = (center2+range2)*np.pi

graphResx = graphRes*((end1-start1)/np.pi) # number of pixels along each axis
graphResy = graphRes*((end2-start2)/np.pi)
if graphResx >= graphResy:
    graphResy *= graphRes/graphResx
    graphResx *= graphRes / graphResx
else:
    graphResx *= graphRes / graphResy
    graphResy *= graphRes / graphResy
graphResx = int(graphResx)
graphResy = int(graphResy)

#pendulum sym
@njit
def simulate_pendulum(theta1, theta2):
    #change
    omega1 = 0.0
    omega2 = 0.0
    #steps
    steps = int(totalTime / dt)
    for k in range(steps):
        #equations stuff you really don't care about this math
        d = theta1 - theta2
        s = np.sin(d)
        c = np.cos(d)
        sin1 = np.sin(theta1)
        sin2 = np.sin(theta2)
        cos1 = np.cos(theta1)
        omega1s = omega1 * omega1
        omega2s = omega2 * omega2
        denom = 2 - c * c
        alpha1 = (-g * (2 * sin1 - sin2 * c) - s * (omega2s + omega1s * c)) / denom
        alpha2 = (2 * s * (omega1s + g * cos1 + omega2s * c)) / denom
        omega1 += alpha1 * dt
        omega2 += alpha2 * dt
        theta1 += omega1 * dt
        theta2 += omega2 * dt
        #if it flips (ts means it's chaotic)
        if abs(theta2) >= np.pi:
            return k * dt
    #if it didn't flip
    return -1.0

@njit(parallel=True)
#making the map
def generate_chaos_map(graphResx,graphResy):
    #empty graph
    data = np.zeros((graphResx, graphResy, 3), dtype=np.uint8)

    #at some point while coding this, the x and y axes swapped so take any "x" or "y" with a grain of salt
    
    #precomputing the range for theta to make save on computation (otherwise you'd have to do these calcs a lot of times each)
    theta_vals_x = np.linspace(start1, end1, graphResx)
    theta_vals_y = np.linspace(start2, end2, graphResy)
    #for the printout
    percent_count = 0
    #for the y
    for i in prange(graphResy):
        #sets theta1 for this column
        theta1 = theta_vals_y[i]
        #for the x
        for j in range(graphResx):
            #sets theta2 for this row
            theta2 = theta_vals_x[j]
            #simulates the pendulum
            flip_time = simulate_pendulum(theta1, theta2)
            #if it didnt flip
            if flip_time < 0:
                #adds color to graph
                data[graphResx-1-j, i, 0] = safe_color[0]
                data[graphResx-1-j, i, 1] = safe_color[1]
                data[graphResx-1-j, i, 2] = safe_color[2]
            #if it was chaotic
            else:
                #how long it took
                normalized = (flip_time / totalTime)**0.5
                #associates time with a color
                h = 270 * normalized
                #converts hsl (360,100,100) to rgb (255)
                x = 1 - abs((h / 60) % 2 - 1)
                if h < 60:
                    rp, gp, bp = 1.0, x, 0.0
                elif h < 120:
                    rp, gp, bp = x, 1.0, 0.0
                elif h < 180:
                    rp, gp, bp = 0.0, 1.0, x
                elif h < 240:
                    rp, gp, bp = 0.0, x, 1.0
                else:
                    rp, gp, bp = x, 0.0, 1.0
                #adds color to graph
                data[graphResx-1-j, i, 0] = int(rp * 255)
                data[graphResx-1-j, i, 1] = int(gp * 255)
                data[graphResx-1-j, i, 2] = int(bp * 255)
        #does the percent printout
        if(do_printout):
            percent_count += 1
            print(1000 * percent_count / graphResy, "%")
    return data

#for computing the runtime
start_time = time.time()
#makes the graph
data = generate_chaos_map(graphResx,graphResy)
#prints the time
if(do_printout):
    print()
    print("Elapsed:",time.time()-start_time)
#makes the image
Image.fromarray(data).show()
