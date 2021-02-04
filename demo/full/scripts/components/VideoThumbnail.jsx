import React from "react";
import VideoThumbnailLoader, {
  DASH_LOADER
} from "../../../../src/experimental/tools/videoThumbnailLoader";

/**
 * React Component which Displays a video thumbnail centered and on top
 * of the position wanted.
 *
 * Takes 2 props:
 *   - {Object} Adaptation - The adaptation that carries the thumbnail track
 *   - {number} Image time - The media time of the image to display
 *
 * @class VideoThumbnailTip
 */
class VideoThumbnail extends React.Component {
  constructor(...args) {
    super(...args);
    this.positionIsCorrected = false;
    this.state = {
      style: {},
    };
    this.isSettingTime = false;
    this._videoElement = undefined;
  }

  correctImagePosition() {
    if (this.positionIsCorrected) {
      return;
    }
    const { xPosition } = this.props;

    if (isNaN(+xPosition) || !this.element) {
      return null;
    }

    const style = {
      transform: `translate(${xPosition}px, -136px)`,
    };

    this.positionIsCorrected = true;
    this.setState({ style });
  }

  componentWillReceiveProps() {
    this.positionIsCorrected = false;
  }

  componentDidMount() {
    this.correctImagePosition();
    if (this._videoElement !== undefined) {
      const fakePlayer = {
        getManifest: () => this.props.manifest,
      };
      this.videoThumbnailLoader =
        new VideoThumbnailLoader(this._videoElement, fakePlayer);
      this.videoThumbnailLoader.addLoader(DASH_LOADER);
    }
  }

  componentDidUpdate() {
    this.correctImagePosition();
  }

  componentWillUnmount() {
    if (this.videoThumbnailLoader) {
      this.videoThumbnailLoader.dispose();
    }
    this._videoElement = undefined;
  }

  render() {
    const { style } = this.state;

    if (this.videoThumbnailLoader) {
      const { time } = this.props;
      if (!this.isSettingTime) {
        this.isSettingTime = true;
        this.videoThumbnailLoader.setTime(Math.floor(time))
          .then(() => this.isSettingTime = false)
          .catch(() => this.isSettingTime = false);
      }
    }

    const divToDisplay = <div
      className="thumbnail-wrapper"
      style={style}
      ref={el => this.element = el}
    >
      <video ref={(videoElement) => {
        if (videoElement !== null) {
          this._videoElement = videoElement;
        }
      }}></video>
    </div>;

    return (
      divToDisplay
    );
  }
}

export default VideoThumbnail;
