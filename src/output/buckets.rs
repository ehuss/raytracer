#[derive(Debug)]
pub enum BucketStrategy {
    Linear,
    Spiral,
}


#[derive(Debug, Eq, PartialEq)]
pub struct Bucket {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}


pub fn create_buckets(image_width: u32,
           image_height: u32,
           strategy: BucketStrategy,
           bucket_width: u32,
           bucket_height: u32)
           -> Vec<Bucket>
{
    match strategy {
        BucketStrategy::Linear => {
            let buckets_x = image_width / bucket_width;
            let buckets_y = image_height / bucket_height;
            let last_x = image_width % bucket_width;
            let last_y = image_height % bucket_height;
            let mut buckets = Vec::with_capacity((buckets_x * buckets_y) as usize);
            {
                let mut add_row = |y, row_height| {
                    for x in 0..buckets_x {
                        buckets.push(Bucket {
                            x: x * bucket_width,
                            y: y * bucket_height,
                            width: bucket_width,
                            height: row_height,
                        });
                    }
                    if last_x != 0 {
                        buckets.push(Bucket {
                            x: buckets_x * bucket_width,
                            y: y * bucket_height,
                            width: last_x,
                            height: row_height,
                        })
                    }
                };
                for y in 0..buckets_y {
                    add_row(y, bucket_height);
                }
                if last_y != 0 {
                    add_row(buckets_y, last_y);
                }
            }
            return buckets;
        },
        BucketStrategy::Spiral => {
            let buckets_x = image_width / bucket_width;
            let buckets_y = image_height / bucket_height;
            let last_x = image_width % bucket_width;
            let last_y = image_height % bucket_height;
            let num_buckets = (buckets_x * buckets_y) as usize;
            let mut buckets = Vec::with_capacity(num_buckets);
            let mut used = vec![vec![false; buckets_x as usize]; buckets_y as usize];
            // Pick a bucket roughly in the middle to start with.
            let mut spiral_x = buckets_x / 2;
            let mut spiral_y = buckets_y / 2;
            let mut spiral_width = 3;
            let mut spiral_height = 3;
            // Hmm, for some reason, the closure capture of "buckets" persists
            // until the end of the scope.  Passing in as argument seems ok.
            let mut add_bucket = |x: u32, y: u32, buckets: &mut Vec<Bucket>| {
                if y < buckets_y && x < buckets_x && !used[y as usize][x as usize] {
                    buckets.push(Bucket {
                        x: x * bucket_width,
                        y: y * bucket_height,
                        width: bucket_width,
                        height: bucket_height,
                    });
                    used[y as usize][x as usize] = true;
                }
            };
            // Add initial bucket.
            add_bucket(spiral_x, spiral_y, &mut buckets);
            spiral_x += 1;
            while buckets.len() < num_buckets {
                // Vertical down right.
                for y in 0..spiral_height-1 {
                    add_bucket(spiral_x, spiral_y+y, &mut buckets);
                }
                // Horizontal across bottom.
                for x in 0..spiral_width-1 {
                    add_bucket(spiral_x.saturating_sub(x).saturating_sub(1),
                               (spiral_y+spiral_height).saturating_sub(2), &mut buckets);
                }
                // Vertical up left.
                for y in 0..spiral_height-1 {
                    add_bucket((spiral_x+1).saturating_sub(spiral_width),
                               (spiral_y+spiral_height).saturating_sub(3).saturating_sub(y),
                               &mut buckets);
                }
                // Horizontal across top.
                for x in 0..spiral_width-1 {
                    add_bucket((spiral_x+2+x).saturating_sub(spiral_width),
                               spiral_y.saturating_sub(1), &mut buckets);
                }
                spiral_x += 1;
                spiral_y = spiral_y.saturating_sub(1);
                spiral_width += 2;
                spiral_height += 2;
            }
            if last_x != 0 {
                for y in 0..buckets_y {
                    buckets.push(Bucket {
                        x: buckets_x*bucket_width,
                        y: y*bucket_height,
                        width: last_x,
                        height: bucket_height,
                    });
                }
            }
            if last_x != 0 && last_y != 0 {
                buckets.push(Bucket {
                    x: buckets_x*bucket_width,
                    y: buckets_y*bucket_height,
                    width: last_x,
                    height: last_y,
                });
            }
            if last_y != 0 {
                for x in (0..buckets_x).rev() {
                    buckets.push(Bucket {
                        x: x*bucket_width,
                        y: buckets_y*bucket_height,
                        width: bucket_width,
                        height: last_y,
                    });
                }
            }
            return buckets;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spiral() {
        // Single bucket.
        assert_eq!(create_buckets(64, 64, BucketStrategy::Spiral, 64, 64),
            vec![Bucket{x:0, y:0, width: 64, height: 64}]);
        // Single bucket, bigger than image.
        assert_eq!(create_buckets(64, 64, BucketStrategy::Spiral, 65, 65),
            vec![Bucket{x:0, y:0, width: 64, height: 64}]);
        // One full bucket, extra on sides.
        assert_eq!(create_buckets(65, 65, BucketStrategy::Spiral, 64, 64),
            vec![Bucket{x:0, y:0, width: 64, height: 64},
                 Bucket{x:64, y:0, width: 1, height: 64},
                 Bucket{x:64, y:64, width: 1, height: 1},
                 Bucket{x:0, y:64, width: 64, height: 1}]);
        // Full test.
        let s = 64;
        assert_eq!(create_buckets(321, 321, BucketStrategy::Spiral, 64, 64),
            vec![Bucket { x: s*2, y: s*2, width: 64, height: 64 },
                 Bucket { x: s*3, y: s*2, width: 64, height: 64 },
                 Bucket { x: s*3, y: s*3, width: 64, height: 64 },
                 Bucket { x: s*2, y: s*3, width: 64, height: 64 },
                 Bucket { x: s,   y: s*3, width: 64, height: 64 },
                 Bucket { x: s,   y: s*2, width: 64, height: 64 },
                 Bucket { x: s,   y: s,   width: 64, height: 64 },
                 Bucket { x: s*2, y: s,   width: 64, height: 64 },
                 Bucket { x: s*3, y: s,   width: 64, height: 64 },
                 Bucket { x: s*4, y: s,   width: 64, height: 64 },
                 Bucket { x: s*4, y: s*2, width: 64, height: 64 },
                 Bucket { x: s*4, y: s*3, width: 64, height: 64 },
                 Bucket { x: s*4, y: s*4, width: 64, height: 64 },
                 Bucket { x: s*3, y: s*4, width: 64, height: 64 },
                 Bucket { x: s*2, y: s*4, width: 64, height: 64 },
                 Bucket { x: s,   y: s*4, width: 64, height: 64 },
                 Bucket { x: 0,   y: s*4, width: 64, height: 64 },
                 Bucket { x: 0,   y: s*3, width: 64, height: 64 },
                 Bucket { x: 0,   y: s*2, width: 64, height: 64 },
                 Bucket { x: 0,   y: s,   width: 64, height: 64 },
                 Bucket { x: 0,   y: 0,   width: 64, height: 64 },
                 Bucket { x: s,   y: 0,   width: 64, height: 64 },
                 Bucket { x: s*2, y: 0,   width: 64, height: 64 },
                 Bucket { x: s*3, y: 0,   width: 64, height: 64 },
                 Bucket { x: s*4, y: 0,   width: 64, height: 64 },
                 Bucket { x: s*5, y: 0,   width: 1, height: 64 },
                 Bucket { x: s*5, y: s,   width: 1, height: 64 },
                 Bucket { x: s*5, y: s*2, width: 1, height: 64 },
                 Bucket { x: s*5, y: s*3, width: 1, height: 64 },
                 Bucket { x: s*5, y: s*4, width: 1, height: 64 },
                 Bucket { x: s*5, y: s*5, width: 1, height: 1 },
                 Bucket { x: s*4, y: s*5, width: 64, height: 1 },
                 Bucket { x: s*3, y: s*5, width: 64, height: 1 },
                 Bucket { x: s*2, y: s*5, width: 64, height: 1 },
                 Bucket { x: s,   y: s*5, width: 64, height: 1 },
                 Bucket { x: 0,   y: s*5, width: 64, height: 1 }]);

    }
}
