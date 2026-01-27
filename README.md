# Double-Pendulum-Chaos-Simulator
Graph of the chaos of double pendulums
Generates a graph
    x=theta2, y=theta1, color represents the time until chaos
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
          
    
