#include <stdio.h>
#include "capture.h"

@implementation CaptureVideoDataOutputSampleBufferDelegate
    
static char * frame;
static NSLock * _nsLock;
static NSArray * observers;

- (void)captureOutput:(AVCaptureOutput *)captureOutput
    didOutputSampleBuffer:(CMSampleBufferRef)sampleBuffer
    fromConnection:(AVCaptureConnection *)connection
{
    void *baseAddress;
    CVImageBufferRef imageBuffer;
    size_t bytesPerRow, width, height, frameBytes;

    imageBuffer = CMSampleBufferGetImageBuffer(sampleBuffer);

    bytesPerRow = CVPixelBufferGetBytesPerRow(imageBuffer);
    width = CVPixelBufferGetWidth(imageBuffer);
    height = CVPixelBufferGetHeight(imageBuffer);

    CVPixelBufferLockBaseAddress(imageBuffer, 0);

    OSType pixelFormat = CVPixelBufferGetPixelFormatType(imageBuffer);
    
    if(pixelFormat != kCVPixelFormatType_32BGRA)
    {
        printf("[Error]: Check format!\n");
        return;
    }

    baseAddress = CVPixelBufferGetBaseAddress(imageBuffer);
    frameBytes = bytesPerRow * height;

    if(height != 720)
    {
        printf("[Error]: Check dimensions!\n");
        return;
    }
        
    [_nsLock lock];

    memcpy(frame, baseAddress, frameBytes);

    [_nsLock unlock];

    CVPixelBufferUnlockBaseAddress(imageBuffer, 0);
}

- (void)readFrame:(void *)dest
{
    [_nsLock lock];
    
    memcpy(dest, frame, 1280 * 720 * 4);
    
    [_nsLock unlock];
}

- (void)settings:(AVCaptureVideoDataOutput *)output
{
    output.videoSettings =
        [NSDictionary dictionaryWithObject:
            [NSNumber numberWithInt:kCVPixelFormatType_32BGRA]
            forKey:(id)kCVPixelBufferPixelFormatTypeKey];
}

- (id)init
{
    self = [super init];
    
    if(self) {
        _nsLock = [[NSLock alloc] init];

        NSNotificationCenter *notificationCenter = [NSNotificationCenter defaultCenter];
        id deviceWasConnectedObserver =
            [notificationCenter addObserverForName:AVCaptureDeviceWasConnectedNotification
            object:nil
            queue:[NSOperationQueue mainQueue]
            usingBlock:^(NSNotification *note) {

            }];
        observers = [[NSArray alloc] initWithObjects:deviceWasConnectedObserver, nil];
    }

    frame = malloc(1280*720*4);
    
    return self;
}
@end