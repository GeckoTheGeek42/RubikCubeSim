class Matrix333(object):

    def __init__(self,intl=None):
        A=[intl,intl,intl]
        B=A+A+A
        self.elements=B+B+B

    def get_element(self,x,y,z):
        return self.elements[x*9+y*3+z]

    def set_element(self,x,y,z,value):
        self.elements[x*9+y*3+z]=value

    def __str__(self):
        s= "0: [%s,%s,%s] 1: [%s,%s,%s]  2: [%s,%s,%s]\n" % \
        (self.get_element(0,0,0),self.get_element(0,0,1),self.get_element(0,0,2),
        self.get_element(1,0,0),self.get_element(1,0,1),self.get_element(1,0,2),
        self.get_element(2,0,0),self.get_element(2,0,1),self.get_element(2,0,2))
        s+="   [%s,%s,%s]    [%s,%s,%s]     [%s,%s,%s]\n" % \
        (self.get_element(0,1,0),self.get_element(0,1,1),self.get_element(0,1,2),
        self.get_element(1,1,0),self.get_element(1,1,1),self.get_element(1,1,2),
        self.get_element(2,1,0),self.get_element(2,1,1),self.get_element(2,1,2))
        s+="   [%s,%s,%s]    [%s,%s,%s]     [%s,%s,%s]" % \
        (self.get_element(0,2,0),self.get_element(0,2,1),self.get_element(0,2,2),
        self.get_element(1,2,0),self.get_element(1,2,1),self.get_element(1,2,2),
        self.get_element(2,2,0),self.get_element(2,2,1),self.get_element(2,2,2))
        return s

    def __getitem__(self,tuple):
        return self.get_element(*tuple)

    def __setitem__(self,tuple,value):
        self.set_element(tuple[0],tuple[1],tuple[2],value)


