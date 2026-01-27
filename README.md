# Double-Pendulum-Chaos-Simulator
Graph of the chaos of double pendulums




IT IS HIGHLY RECOMMENDED TO RUN THIS ON A PROGRAM THAT RUNS ON YOUR COMPUTER, RATHER THAN ON WEB

Generates a graph
	
	
	x=theta2, y=theta1, color represents the time until chaos
        the color is proportional to how long it takes for a pendulum to fall into chaos
            using hsl hue values
        white represents a pendulum that does not become chaotic
        red is used for pendulums that fall into chaos quickly, while blue and purple are for those that fall near the end of the simulation
        the colors are mapped to the total time the simulation was run for
        they are skewed by finding the square root of the percentage of the time it took to become chaotic
            (flip_time/total_time)**0.5
            this has the effect of concentrating the colors more towards the latter end of the simulation in terms of time
                this is necessary because the majority of pendulums fall into chaos quickly, so this increases the amount of meaningful data without changing relative comparisons between points
        note that running for the simulation for a large amount of steps can, eventually, concentrate the colors too closely together as almost all the events will happen toward the beginning
            in my experience, this has never happened, but i have never extensively used the code beyond approximately totalTime = 100 while dt = 0.01 (10000 steps)
    Note that (-pi,-pi) and (pi,pi) should be used as boundaries for the graph
    Note that (-pi,-pi) would be on the bottom left of the graph (meaning that the axes increase as you go right and as you go up)
    If your graph covers more area than from (-pi,pi) to (pi,pi), the code will function fine and an accurate graph will still be generated
        this is not recommended, however, as the relevant graph tiles, so anything outside (-pi,-pi),(pi,pi) will be redundant information

Use graphRes to change the target resolution


    the specified number will be the larger of the two dimensions if the width and height of the ranges are not equal
    The resolution is exponentially (O(n^2)) proportional with runtime

Use center1 and center2 to specify the center of each axis


    each variable should range from [-1,1], as they are multiplied by pi
    e.x. if you wanted the center of the theta1 axis to be at (1/2)*pi, you would set center1=1/2
    independent from runtime

Use range1 and range2 to specify the radius of each axis


    1/2 the width of each axis
    each variable should range from [-1,1], as they are multiplied by pi
    e.x. if you set center1=1/2 and you wanted the range to go from [0,pi], you would set range1 = 1/2
    independent from runtime

Use totalTime to change how long it will check each pendulum


    Note that the variable dt is the step size
        It will run each pendulum for int(totalTime/dt) steps
    totalTime is generally linearly proportional to the runtime
    a smaller totalTime results in a graph where some pendulums represented as not becoming chaotic may actually be chaotic
        this is generally not the largest factor in an accurate graph

Use dt to change the step size


    dt is inversly proportional with runtime 
    a bigger dt will result in a less accurate graph in multiple ways, with both false negatives and false positives
    this is generally the most impactful variable in regards to accuracy

Use g to change the value of gravity


    independent from runtime
    a smaller g will produce more accurate results at the cost of taking more steps to find chaotic pendulums
        if you use a smaller value for g, it is recommended to either increase either totalTime or decrease dt to accomadate
        therefore, while g is independent from runtime on paper, in practice it is somewhat proportional so long as you want an accurate graph
    too large values for g will produce inaccurate results, although generally skewed towards more false chaos
          it is recommended not to tweak the g value, as the desired changes to the graph can generally be achieved by tweaking dt or totalTime

Use safe_color to change the rgb (255) value used to represent pendulums which do not flip


    it is recommended to keep this value greyscale to not makes the chaotic pendulums on the graph easier to read

The printout can be toggled using do_printout


    if enabled the printout will print the progress of the program in %
            this is fairly inaccurate and should only be trusted to be accurate within about 20%, but it's better than nothing
                it is inaccurate because the program uses njit to run threads in parrallel, so they do not all start or finish in sync as some end early once a pendulum becomes chaotic
            it's less accurate the longer you are trying to simulate for
                the longer each thread takes, the more out of sync they get
            for simulations under a minute it's pretty good
    once it is done it will also print how long the simulation took, in seconds

The graph will be created as a png from an array of rgb (255) colors


    it is recommended that you test the program once on a small resolution (so that it runs fast) to make sure the method of creating the png works for your system, as it hasn't been tested on anything other than mac using pycharm
    note these pngs are not automatically saved, so save them before you close them if you want to open them again without rerunning the simulation
    i'm sure you can figure out some other way on your own to generate an image from the array, given it already has all the colors in it in rgb


    
