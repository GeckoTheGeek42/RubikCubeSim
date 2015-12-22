from OpenGL.GL import *
from OpenGL.GLU import *
import pygame
from pygame.locals import *

def gl_init(screenx, screeny):
    video_flags = HWSURFACE|OPENGL|DOUBLEBUF
    surface = pygame.display.set_mode((screenx,screeny), video_flags)
    resize((screenx, screeny))
    init()

def resize((width, height)):
    if height==0:
        height=1
    glViewport(0, 0, width, height)
    glMatrixMode(GL_PROJECTION)
    glLoadIdentity()
    gluPerspective(45, 1.0*width/height, 0.1, 100.0)
    glMatrixMode(GL_MODELVIEW)
    glLoadIdentity()

def init():
    glEnable(GL_TEXTURE_2D)
    glEnable(GL_BLEND)
    glBlendFunc(GL_SRC_ALPHA, GL_ONE_MINUS_SRC_ALPHA)
    glTexEnvi(GL_TEXTURE_ENV, GL_TEXTURE_ENV_MODE, GL_MODULATE)
        
    glShadeModel(GL_SMOOTH)
    glClearColor(0.0, 0.0, 0.0, 0.0)
    glClearDepth(1.0)
    glEnable(GL_DEPTH_TEST)
    glEnable( GL_ALPHA_TEST )
    glDepthFunc(GL_LEQUAL)
    glHint(GL_PERSPECTIVE_CORRECTION_HINT, GL_NICEST)
    
    LightAmbient  = ( (0.5, 0.5, 0.5, 1.0) );
    LightDiffuse  = ( (1.0, 1.0, 1.0, 1.0) );
    LightPosition = ( (1250.0, 0.0, 1250.0, 1.0) );
    glLightfv( GL_LIGHT0, GL_AMBIENT, LightAmbient )
    glLightfv( GL_LIGHT0, GL_DIFFUSE, LightDiffuse )
    glLightfv( GL_LIGHT0, GL_POSITION, LightPosition )
    glEnable( GL_LIGHT0 )
