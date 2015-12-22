import os
from OpenGL.GL import *
from OpenGL.GLU import *

from matrix333 import *
from gameobjects.matrix44 import *

import pygame

def load_texture(filename):
    texture_path=os.path.join('Images',filename)
    texture_surface=pygame.image.load(texture_path)
    texture_data=pygame.image.tostring(texture_surface,"RGBA", True)
    width=texture_surface.get_width()
    height=texture_surface.get_height()

    material_texture=glGenTextures(1)

    glBindTexture(GL_TEXTURE_2D, material_texture)
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR)
    glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR)
    
    glTexImage2D( GL_TEXTURE_2D, 0, GL_RGBA, width, height, 0,
                  GL_RGBA, GL_UNSIGNED_BYTE, texture_data )
    return int(material_texture)

def use_texture(texture_id):
    glBindTexture(GL_TEXTURE_2D,texture_id)
    
def build_cube(side_colors):
    list_id=glGenLists(1)
    glNewList(list_id, GL_COMPILE)

    use_texture(side_colors[0])
    glBegin(GL_QUADS)
    glTexCoord2f(0.0, 0.0); glVertex3f(-0.5, -0.5, -0.5)
    glTexCoord2f(1.0, 0.0); glVertex3f(0.5, -0.5, -0.5)
    glTexCoord2f(1.0, 1.0); glVertex3f(0.5, 0.5, -0.5)
    glTexCoord2f(0.0, 1.0); glVertex3f(-0.5, 0.5, -0.5)
    glEnd()
    use_texture(side_colors[1])
    glBegin(GL_QUADS)
    glTexCoord2f(0.0, 0.0); glVertex3f(0.5, -0.5, -0.5)
    glTexCoord2f(1.0, 0.0); glVertex3f(0.5, -0.5, 0.5)
    glTexCoord2f(1.0, 1.0); glVertex3f(0.5, 0.5, 0.5)
    glTexCoord2f(0.0, 1.0); glVertex3f(0.5, 0.5, -0.5)
    glEnd()
    use_texture(side_colors[2])
    glBegin(GL_QUADS)
    glTexCoord2f(0.0, 0.0); glVertex3f(0.5, -0.5, 0.5)
    glTexCoord2f(1.0, 0.0); glVertex3f(-0.5, -0.5, 0.5)
    glTexCoord2f(1.0, 1.0); glVertex3f(-0.5, 0.5, 0.5)
    glTexCoord2f(0.0, 1.0); glVertex3f(0.5, 0.5, 0.5)
    glEnd()
    use_texture(side_colors[3])
    glBegin(GL_QUADS)
    glTexCoord2f(0.0, 0.0); glVertex3f(-0.5, -0.5, 0.5)
    glTexCoord2f(1.0, 0.0); glVertex3f(-0.5, -0.5, -0.5)
    glTexCoord2f(1.0, 1.0); glVertex3f(-0.5, 0.5, -0.5)
    glTexCoord2f(0.0, 1.0); glVertex3f(-0.5, 0.5, 0.5)
    glEnd()
    use_texture(side_colors[4])
    glBegin(GL_QUADS)
    glTexCoord2f(0.0, 0.0); glVertex3f(-0.5, 0.5, -0.5)
    glTexCoord2f(1.0, 0.0); glVertex3f(0.5, 0.5, -0.5)
    glTexCoord2f(1.0, 1.0); glVertex3f(0.5, 0.5, 0.5)
    glTexCoord2f(0.0, 1.0); glVertex3f(-0.5, 0.5, 0.5)
    glEnd()
    use_texture(side_colors[5])
    glBegin(GL_QUADS)
    glTexCoord2f(0.0, 0.0); glVertex3f(-0.5, -0.5, -0.5)
    glTexCoord2f(1.0, 0.0); glVertex3f(0.5, -0.5, -0.5)
    glTexCoord2f(1.0, 1.0); glVertex3f(0.5, -0.5, 0.5)
    glTexCoord2f(0.0, 1.0); glVertex3f(-0.5, -0.5, 0.5)
    glEnd()
    glEndList()
    return int(list_id)

def build_cube_lists():

    Red=load_texture('Red256.png'); Orange=load_texture('Orange256.png')
    Yellow=load_texture('Yellow256.png'); Green=load_texture('Green256.png')
    Blue=load_texture('Blue256.png'); Purple=load_texture('White256.png')
    Notex=load_texture('None64.png')

    cube_lists=Matrix333(0.)
    
    for i in xrange(3):
        for j in xrange(3):
            for k in xrange(3):
                side_colors=[Red,Blue, Orange,Green, Purple, Yellow]
                if i<2:
                    side_colors[1]=Notex
                if i>0:
                    side_colors[3]=Notex
                if j<2:
                    side_colors[4]=Notex
                if j>0:
                    side_colors[5]=Notex
                if k<2:
                    side_colors[2]=Notex
                if k>0:
                    side_colors[0]=Notex
                cube_lists.set_element(i,j,k,
                        build_cube(side_colors))

    return cube_lists
 
class RubiksCube(object):

    def __init__(self):
        self.sides=build_cube_lists() # returns a Matrix333 object
        identity=Matrix44()
        self.rotations=Matrix333(identity)

    def draw(self,x_angle,y_angle,view_distance,viewmode,
             angle=0,sidex=-99,sidey=-99,sidez=-99):
        glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT)
        glLoadIdentity()
        glTranslatef(0.0, 0.0, -view_distance)
        glRotatef(x_angle, 1.0, 0.0, 0.0)
        glRotatef(y_angle, 0.0, 1.0, 0.0)
    #    glEnable(GL_LIGHTING)

        for i in xrange(-1,2):
            for j in xrange(-1,2):
                for k in xrange(-1,2):
                    if not ((i==0) and (j==0) and (k==0)):
#                    if ((i==0) and (j==1) and (k==0)):
                        glPushMatrix()
                        if (i==sidex) and (sidex!=0):
                            glRotatef(angle,1.,0.,0.)
                        if (j==sidey) and (sidey!=0):
                            glRotatef(angle,0.,1.,0.)
                        if (k==sidez) and (sidez!=0):
                            glRotatef(angle,0.,0.,1.)
                        glTranslatef(i*viewmode, j*viewmode, k*viewmode)
                        rotation=self.rotations[i+1,j+1,k+1].to_opengl()
                        glMultMatrixf(rotation)
                        glCallList(self.sides[i+1,j+1,k+1])
                        glPopMatrix()

    #    glDisable(GL_LIGHTING)

    def find_cubie(self,cubie_list_id):
        for i in xrange(3):
            for j in xrange(3):
                for k in xrange(3):
                    if self.sides[i,j,k]==cubie_list_id:
                        return (i,j,k)
        return "Error"
                    
                        

