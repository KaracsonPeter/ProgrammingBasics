## What is a neural network (NN) in my understanding?
One specific group of algorithms in set of Machine Learning algorithm.  
1. A structure of [1] **neurons** and their connections. It performs mathematical operations to produce some output which makes sense for some reason.
2. By adding the appropriate input to the network at a given time point, an output appears with a delta *t* time accordingly.
3. During learning process, you are [2] **adjusting weights and biases** to channel the appropriate feature (information) to the appropriate processor (probably a neuron which is usually in the next layer.)

**All in all** NNs are the infrastructure of understanding.  
They allows:
 - The right amount of specific information 
 - To be arrived at exact places
 - At appropriate time for further processing.

[1] **Neurons:** Base building blocks of NNs.  
A "thing" which is able to process numerical information present in its inputs by:
 1. **choosing relevant input information** (aka feature) according to mathematical rules such as:
    - Multiplying by a weight (*which is a learnt parameter*)
    - Choosing the most relevant input e.g. by taking maximum (*called max pooling*)
    - etc.
 2. **combine these chosen information together** in some mathematical way, 
    that this combination represents another useful information for further information processing.  
    Accordingly, I assume, if I taught a neuron in a NN, and there is no such situation, 
    when a neuron significantly listens to it, it worth nothing to be present in the network.

[2] **Adjusting weights and biases**  
1. Okay, but how do we know into which direction we should modify weights and biases?  
An **error** between the *actual output* and the *desired output* for the currently tested training example is **calculated**, using a so-called [3] **loss function**.
2. The problem is that the outputs of the neurons in different layers are interdependent (it depends on the weights from the previous layer), 
so it can be difficult to determine what should be the desired output for the actual problem.
3. Unfortunately, to overcome this problem, the loss function must be differentiated in order to determine which direction of weight adjustments improve problem-solving for the current situation.
4. Yeees, but loss functions can be real hard to differentiate depending on what activation function is applied on the sum of the weighted input features in them.
Therefore, the first applied activation function was the sigmoid (for a method called the back-propagation), which is easy to differentiate.
5. Finally, optimization algorithms, such as gradient descent, are used to determine the size of the weight adjustments,
in order to minimize the loss function for the next time we are facing a really similar (or the same) problem to be solved.

[3] **loss function**  
For every specific input of a **[5] Machine Learning** (ML) model, an actual output belongs to. A loss function can be calculated for this output.
This loss function represents, the error aka, how far we are from the desired output compared to the actual output related to the specific input.  
There are several types loss functions:
 - Mean Squared Error (MSE)
 - Mean Squared Logarithmic Error (MSLE)
 - Mean Absolute Error (MAE)
 - etc. ([search for more](https://www.analyticsvidhya.com/blog/2022/06/understanding-loss-function-in-deep-learning/))

In a specific type of ML called NN, the loss function is dependent on 
 - the activation function of the neuron,
 - and the weights related to the inputs of the neuron,
since the output of the neuron is taking the activation function for the weighted sum of some other outputs from the previous layer.  

E.g.: Out_layer_2 = b_layer_2 + RELU(weight_layer_21_n1 * Out_layer_1_neuron_1 + weight_layer_21_n2 * Out_layer_1_neuron_2 + ... + weight_layer_21_nn * Out_layer_1_neuron_n)

[4] [**Cost function**](https://www.analyticsvidhya.com/blog/2021/02/cost-function-is-no-rocket-science/)  
The cost function is a scalar value that summarizes the overall performance of a NN for a given dataset (e.g. test or validation). 
It is calculated as the average (or sum) of the loss functions of each instance in the dataset.

[5] **Machine Learning** (ML)  
There are several types of ML:
 - Linear Regression ([Least squares](https://www.youtube.com/watch?v=PaFPbb66DxQ))
 - Non-Linear Regression
 - Polynomial Regression
 - Logistic Regression
 - Decision Trees
 - Random Forests
 - Support Vector Machines (SVM)
 - **Neural Networks**