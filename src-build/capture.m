#include <stdio.h>
#import <AVFoundation/AVFoundation.h>
#import <Foundation/Foundation.h>

@interface CaptureVideoDataOutputSampleBufferDelegate : NSObject <AVCaptureVideoDataOutputSampleBufferDelegate> {
    char * frame;
    NSLock * nsLock;
    NSArray * observers;
}

@end

@implementation CaptureVideoDataOutputSampleBufferDelegate

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
        
    [nsLock lock];

    memcpy(frame, baseAddress, frameBytes);

    [nsLock unlock];

    CVPixelBufferUnlockBaseAddress(imageBuffer, 0);
}

- (id)initWithWidth:(size_t)width withHeight:(size_t)height withChannels:(size_t)nchannels
{
    self = [super init];
    
    if(self) {
        nsLock = [[NSLock alloc] init];

        NSNotificationCenter *notificationCenter = [NSNotificationCenter defaultCenter];
        id deviceWasConnectedObserver =
            [notificationCenter addObserverForName:AVCaptureDeviceWasConnectedNotification
            object:nil
            queue:[NSOperationQueue mainQueue]
            usingBlock:^(NSNotification *note) {

            }];
        observers = [[NSArray alloc] initWithObjects:deviceWasConnectedObserver, nil];
    }

    frame = malloc(width * height * nchannels);
    
    return self;
}
@end