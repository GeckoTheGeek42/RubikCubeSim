from OpenGL.GL import *
from cube_object import load_texture, use_texture

class Text_Image(object):
    def __init__(self, filename, x=0.,y=0., height=1.,width=1.):
        texid = load_texture(filename)
        hh=height/2.
        hw=width/2.
        list_id=glGenLists(1)
        glNewList(list_id, GL_COMPILE)
        use_texture(texid)
        glBegin(GL_QUADS)
        glTexCoord2f(0.0, 0.0); glVertex3f(x-hw, y-hh, 0)
        glTexCoord2f(1.0, 0.0); glVertex3f(x+hw, y-hh, 0)
        glTexCoord2f(1.0, 1.0); glVertex3f(x+hw, y+hh, 0)
        glTexCoord2f(0.0, 1.0); glVertex3f(x-hw, y+hh, 0)
        glEnd()
        glEndList()
        self.list_id = list_id


    def draw(self,x_angle,y_angle,view_distance):
#        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        glLoadIdentity()
        glTranslatef(0.0, 0.0, -view_distance/2.)
#        glRotatef(20, 0.0, 1.0, 0.0)
        glScalef(5.0,3.0,5.0)
        glCallList(self.list_id)

