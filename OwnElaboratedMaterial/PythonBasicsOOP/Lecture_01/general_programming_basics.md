# General programming basics
## Introduction
Our new world is based on outsourcing certain tasks of our brain / mind.  
That is why programming languages were constructed / invented / developed by creative engineers.  
They wanted to have the language best fitting to their dreams and personality by which they could dream something. 
1. If you have something constructed in your brain(e.g. your dream app) or just want to try out how things work, 
or getting familiarized with what works and what not in a short amount of time,
2. If you want to learn in fields of sciences where your time is more valuable than the performance of your code,
3. If you are just want to learn some programming basics,  
then Python is the best choice for your learning path.

Before we start actual programming, I want to stick a kind of programming mindset into your brain 
which can help to avoid several weeks or even months of headache in the topics.  
Every important definition and phrase that are essential are marked with **bold letters** in the text.

## Programming language types
There are [too many way](https://softwareengineering.stackexchange.com/questions/17976/how-many-types-of-programming-languages-are-there) you can group programming languages. 
You can think of these groups like *different ways of thinking or approaches to solving problems*.  
Two categories you need to know for this course:

### 1. **Script** programming languages  
   When you are thinking roughly in your life, sometimes you don't want to go into every detail, You only want to stay "high".  
   That is what you can do with script languages. Script languages use a so-called **high-level** approach. 
   Here, you don't have to focus on details, you only need to focus on performing the appropriate task necessary for your purpose.  
   
   There are many **libraries** and **packages** related to these languages. These contain useful tools for your development.
   For example, if you want to build a chair, you don't want to develop a hammer, a saw, different nails, etc. to see that you are able to do that. 
   Just want to use these tools to support your work. That is, why you will **import** e.g. a package called "Workshop" which contains your tools.
   After you hire an entire workshop it is much easier to build a chair from scratch.

   Python is a script language. In it, importing this workshop look like this:
   ```python
    import Workshop
    # or
    import Workshop as ws
   ```
   After one of the lines above you can use your workshop.
   
### 2. Object-Oriented Programming (OOP) languages
   In Object-Oriented Programming languages, problems, like building a chair are broken down into little pieces called **objects**.  
   After starting the task of building a chair, you will immediately notice you have to make 4 `chair legs`, 1 `seating area`, and a `backrest`.
   These are called **objects** in OOP. You may have to make 10 chairs by Friday, which requires 40 legs. That's a lot...   
   After the second leg had been done, you may start to think... Isn't there an easier way than manually sawing and planing the wooden legs one by one?
   So you prepare a template, from which you can cut out legs more easily. This template called **class**, 
   and it contains every information, every parameter necessary to prepare (**instantiate** in OOP) your chair leg **object**.
   If you are a successful "joiner", you will get more and more orders from **customers**. Some of them will order chairs which has the same leg.
   So you have to name your leg classes carefully to help your future work with them. (E.g.: ThinLeg, CurlyLeg, etc.)  
   (This type of naming, which describes the purpose of the object is called the **Hungarian notation**.)
   
   Python is an OOP language as well.  
   The example above:  
   ```python
    class CurlyLeg:
        def __init__(self):
            self.length = 0.5
            self.width = 0.05
   ```

## OOP programming in detail

