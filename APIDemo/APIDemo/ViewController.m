//
//  ViewController.m
//  APIDemo
//
//  Created by Tao Xie on 12/25/12.
//  Copyright (c) 2012 Tao Xie. All rights reserved.
//

#import "ViewController.h"
#import <objc/runtime.h>

@interface ViewController ()

@end

@implementation ViewController

- (void)associateObjectAction {
    static const void *key = "overview_key";
    
    NSArray *array = @[@"One", @"Two", @"Three"];
    NSString *overview = [[NSString alloc] initWithFormat:@"%@", @"First three numbers"];
    
    objc_setAssociatedObject(array, key, overview, OBJC_ASSOCIATION_RETAIN_NONATOMIC);
    NSString *associatedObject = (NSString *)objc_getAssociatedObject(array, key);
    NSLog(@"associatedObject: %@", associatedObject);
    objc_setAssociatedObject(array, key, nil, OBJC_ASSOCIATION_ASSIGN);
}

- (void)viewWillAppear:(BOOL)animated {
    [super viewWillAppear:animated];
    // NSLog(@"self.tryItButton:%@", self.tryItButton);
}
- (void)viewDidLoad
{
    [super viewDidLoad];
    NSLog(@"self.tryItButton:%@", self.tryItButton);
    [self.tryItButton addTarget:self action:@selector(associateObjectAction) forControlEvents:UIControlEventTouchUpInside];
}

- (void)didReceiveMemoryWarning
{
    [super didReceiveMemoryWarning];
    // Dispose of any resources that can be recreated.
}

@end
