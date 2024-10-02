import numpy as np
from scipy.spatial.transform import Rotation
from astropy.coordinates import spherical_to_cartesian, cartesian_to_spherical
from astropy import units as u
import math


def inside_polygon(point, vertices):
    excess = polygon_excess(vertices)
    N = len(vertices)
    lat0, lon0 = point[0], point[1]
    lats, lons = vertices[:, 0], vertices[:, 1]
    transform = Rotation.from_euler("zy", [-lon0, lat0 - 90], degrees=True)
    polygon_cartesian = spherical_to_cartesian(np.ones(N), lats * u.deg, lons * u.deg)
    polygon_cartesian_transformed = transform.apply(np.stack(polygon_cartesian).T)
    xs, ys, zs = [polygon_cartesian_transformed[:, i] for i in range(3)]
    polygon_spherical_transformed = cartesian_to_spherical(xs, ys, zs)

    lons_transformed = polygon_spherical_transformed[2].value

    sum_angle = 0
    flag = False

    for i in range(N - 1):
        dlon = lons_transformed[i + 1] - lons_transformed[i]
        if dlon > np.pi:
            dlon = -2 * np.pi + dlon
        if dlon < -np.pi:
            dlon = 2 * np.pi + dlon
        sum_angle += dlon
    if 0 < excess < 2 * np.pi or excess < -2 * np.pi:
        if np.abs(sum_angle - 2 * np.pi) > 0.1:
            flag = True
        return flag
    elif -2 * np.pi < excess < 0 or excess > 2 * np.pi:
        if np.abs(sum_angle + 2 * np.pi) < 0.1:
            flag = True
        return flag
    else:
        raise Exception("points not valid for polygon")



def polygon_excess(vertices):
    N = len(vertices)

    sum_excess = 0

    for i in range(N - 1):
        p1, p2 = np.radians(vertices[i]), np.radians(vertices[i + 1])
        pdlat, pdlon = p2[0] - p1[0], p2[1] - p1[1]
        dlon = np.abs(pdlon)

        if dlon < 1e-6:
            continue

        if dlon > np.pi:
            dlon = 2 * np.pi - dlon
        if pdlon < -np.pi:
            p2[1] = p2[1] + 2 * np.pi
        if pdlon > np.pi:
            p2[1] = p2[1] - 2 * np.pi
        havb = (1 - np.cos(pdlat)) / 2 + np.cos(p1[0]) * np.cos(p2[0]) * (
            1 - np.cos(dlon)
        ) / 2
        b = 2 * np.arcsin(np.sqrt(havb))
        a, c = np.pi / 2 - p1[0], np.pi / 2 - p2[0]
        s = 0.5 * (a + b + c)
        t = (
            np.tan(s / 2)
            * np.tan((s - a) / 2)
            * np.tan((s - b) / 2)
            * np.tan((s - c) / 2)
        )
        excess = 4 * np.arctan(np.sqrt(np.abs(t)))
        if p2[1] - p1[1] < 0:
            excess = -excess

        sum_excess += excess

    return sum_excess


def distance_km(lat1, lon1, lat2, lon2):
    lon1, lat1, lon2, lat2 = map(math.radians, [lon1, lat1, lon2, lat2])

    dlon = lon2 - lon1
    dlat = lat2 - lat1
    a = (
        math.sin(dlat / 2) ** 2
        + math.cos(lat1) * math.cos(lat2) * math.sin(dlon / 2) ** 2
    )
    c = 2 * math.asin(math.sqrt(a))
    r = 6371
    return c * r


def max_apart(points, apart):
    for i, point in enumerate(points):
        for f, check_point in enumerate(points):
            if i == f:
                continue
            distance = distance_km(
                point[0],
                point[1],
                check_point[0],
                check_point[1],
            )
            if distance > apart:
                return False
    return True


def polygon_area(vertices):
    excess = polygon_excess(vertices)
    area = np.abs(excess)

    if area > 2 * np.pi:
        area = 4 * np.pi - area
    return area
