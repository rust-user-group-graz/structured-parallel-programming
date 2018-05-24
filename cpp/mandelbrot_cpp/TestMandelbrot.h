//
// Created by davidf on 24.05.18.
//

#ifndef MANDELBROT_CPP_TESTMANDELBROT_H
#define MANDELBROT_CPP_TESTMANDELBROT_H

#include <QTest>

class TestMandelbrot: public QObject
{
    Q_OBJECT
private slots:
    void initTestCase()
    { qDebug("called before everything else"); }

    void testMandelbrot();

    void cleanupTestCase()
    { qDebug("called after myFirstTest and mySecondTest"); }
};


#endif //MANDELBROT_CPP_TESTMANDELBROT_H
